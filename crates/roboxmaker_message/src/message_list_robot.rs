use log::*;
use uuid::Uuid;
use yew::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use code_location::code_location;
use crate::message_card::MessageCard;
use crate::{MessageGroupCategory, message_card::MessageEdit};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{user_model, message_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_types::types::{LessonId, PostId, MessageId, GroupId, UserId, RobotId, AppRoute, MyUserProfile};

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MessageAuthor {
    pub user_id: Uuid,
    pub full_name: String,
    pub pic_path: String,
    pub user_staff: Option<Uuid>,
    pub user_teacher: Option<Uuid>,
    pub user_student: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MessageProfile {
    pub message_id: Uuid,
    pub author: MessageAuthor,
    pub content: String,
    pub timestamp: String,
    pub reply_id: Option<Uuid>,
    pub replies_private: bool,
}

pub struct MessageList {
    link: ComponentLink<Self>,
    props: MessageListProperties,
    graphql_task: Option<GraphQLTask>,
    posts_sub: Option<SubscriptionTask>,
    lessons_sub: Option<SubscriptionTask>,
    robots_sub: Option<SubscriptionTask>,
    direct_sub: Option<SubscriptionTask>,
    task_messages: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    messages: Vec<MessageProfile>,
    replying_to: Option<MessageId>,   
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct MessageListProperties {
    pub on_app_route: Option<Callback<AppRoute>>,
    pub user_id: Option<UserId>,
    pub user_profile: Option<MyUserProfile>,
    pub group_category: MessageGroupCategory,   
}

#[derive(Debug)]
pub enum MessageListMessage {
    FetchMessagesByLessonGroup(LessonId, GroupId),
    MessageLessons(Option<message_model::messages_by_lesson_group::ResponseData>),
    FetchMessagesByPostGroup(PostId, GroupId),
    MessagePosts(Option<message_model::messages_by_post_group::ResponseData>),
    FetchMessagesByRobotGroup(RobotId, GroupId),
    MessageRobots(Option<message_model::messages_by_robot_group::ResponseData>),
    FetchMessagesByDirectMessageGroup(GroupId),
    FetchMessagesWithFilesUser,
    MessagesDirect(Option<message_model::messages_by_direct_message_group::ResponseData>),
    MessagesFiles(Option<message_model::contribution_files_by_author_id::ResponseData>),
    ReplyTo(MessageId),
    ExitThread,
    DeleteMessage(MessageId),
    MessageDeleted(Option<message_model::delete_message_by_id::ResponseData>),
}

impl Component for MessageList {
    type Message = MessageListMessage;
    type Properties = MessageListProperties;

    fn create(ctx: &Context<Self>) -> Self {
        match props.group_category {
            MessageGroupCategory::Lessons(group_id, lesson_id) => {
                link().send_message(MessageListMessage::FetchMessagesByLessonGroup(
                    lesson_id, group_id,
                ));
            }
            MessageGroupCategory::Posts(group_id, post_id) => link().send_message(
                MessageListMessage::FetchMessagesByPostGroup(post_id, group_id),
            ),
            MessageGroupCategory::Robots(group_id, robot_id) => link().send_message(
                MessageListMessage::FetchMessagesByRobotGroup(robot_id, group_id),
            ),
            MessageGroupCategory::DirectMessages(group_id) => link().send_message(
                MessageListMessage::FetchMessagesByDirectMessageGroup(group_id),
            ),
            MessageGroupCategory::FilesUser => {
                link().send_message(MessageListMessage::FetchMessagesWithFilesUser)
            }
        }
        MessageList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            posts_sub: None,
            lessons_sub: None,
            robots_sub: None,
            direct_sub: None,
            messages: vec![],
            replying_to: None, 
            task_messages: None,
            delete_task: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MessageListMessage::FetchMessagesByRobotGroup(robot_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::messages_by_robot_group::Variables {
                        group_id: group_id.0,
                        robot_id: robot_id.0,
                    };
                    let task = message_model::MessagesByRobotGroup::subscribe(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessageRobots(response)
                        }
                    );
                    self.robots_sub = Some(task);
                }
            }
            MessageListMessage::MessageRobots(response) => {
                self.messages = response
                    .clone()
                    .and_then(|data| Some(data.message_profile))
                    .clone()
                    .unwrap_or(vec![])
                    .iter()
                    .map(|message| {
                        let author = MessageAuthor {
                            user_id: message.author.clone().and_then(|data| Some(data.user_id)).unwrap_or(Uuid::default()),
                            full_name: message.author.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string()),
                            pic_path: message.author.clone().and_then(|data| data.pic_path).unwrap_or("".to_string()),
                            user_staff: message.author.clone().and_then(|data| data.user_staff.clone()).and_then(|data| Some(data.user_id)),
                            user_teacher: message.author.clone().and_then(|data| data.user_teacher.clone()).and_then(|data| Some(data.user_id)),
                            user_student: message.author.clone().and_then(|data| data.user_student.clone()).and_then(|data| Some(data.user_id)),
                        };
                        MessageProfile {
                            message_id: message.message_id,
                            author,
                            timestamp: message.timestamp.format("%a %b %e %Y").to_string(),
                            reply_id: message.reply_id,
                            replies_private: message.replies_private,
                            content: message.message_content.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string()),
                        }
                    }).collect();
            }
            MessageListMessage::ReplyTo(message_id) => {
                self.replying_to = Some(message_id);
            }
            MessageListMessage::ExitThread => {
                self.replying_to = None;
            }
            MessageListMessage::DeleteMessage(message_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::delete_message_by_id::Variables {
                        message_id: message_id.0,
                    };
                    let task = message_model::DeleteMessageById::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessageDeleted(response)
                        }
                    );
                    self.delete_task = Some(task);
                }
            }
            MessageListMessage::MessageDeleted(_) => {}
        }

        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render= true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_message_delete = ctx.link().callback(|message_id| MessageListMessage::DeleteMessage(message_id));

        let auth_user = ctx.props().auth_user.clone();
        let group_category = ctx.props().group_category;

        let stats = self.messages.iter().fold(
            HashMap::new(),
            |mut stats: HashMap<Uuid, usize>, message_profile| {
                if let Some(reply_id) = message_profile.reply_id {
                    let stat = stats.entry(reply_id).or_insert(0);
                    *stat += 1;
                } else {
                    stats.entry(message_profile.message_id).or_insert(0);
                }
                stats
            },
        );

        let on_reply_to = self
            .link
            .callback(move |message_id| MessageListMessage::ReplyTo(message_id));

        let messages = if let Some(replying_to) = &self.replying_to {
            self
            .messages
            .iter()
            .filter(|message_profile| replying_to.0 == message_profile.message_id || Some(replying_to.0) == message_profile.reply_id)
            .map(|message_profile| {
                info!("{:?} {}", message_profile.reply_id, message_profile.message_id);
                html!{
                    <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                        auth_user=auth_user.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone() 
                        on_message_delete=on_message_delete.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_reply_to=None
                        mod_commets=true
                        replying_to=self.replying_to 
                        stats_messages_reply=stats.clone() />
                }
            })
            .collect::<Html>()
        } else {
            self
            .messages
            .iter()
            .filter(|message_profile| message_profile.reply_id.is_none())
            .map(|message_profile| {
                html! {
                    <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                        auth_user=auth_user.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_reply_to=Some(on_reply_to.clone()) 
                        replying_to=self.replying_to 
                        on_message_delete=on_message_delete.clone()
                        mod_commets=true
                        stats_messages_reply=stats.clone() />
                }
            })
            .collect::<Html>()
        };
        let maybe_add_message = html! {
            <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                auth_user=auth_user.clone() 
                group_category=group_category.clone()
                message_profile=None 
                message_edit_style=MessageEdit::Thread
                on_reply_to=None 
                mod_commets=false
                on_message_delete=on_message_delete.clone()
                replying_to=self.replying_to
                stats_messages_reply=stats.clone() />
        };
        let maybe_exit_thread = if self.replying_to.is_some() {
            let on_exit_thread = ctx.link().callback(move |_| MessageListMessage::ExitThread);
            html! {
                <a class="btn bg-purple-on ms-5" onclick=&on_exit_thread>
                    <span class="text-white">
                        <i class="fas fa-times"></i>
                    </span>
                </a>
            }
        } else {
            html! {}
        };
        let on_reply_to = self
            .link
            .callback(move |message_id| MessageListMessage::ReplyTo(message_id));
        let list_messages = self
            .messages
            .iter()
            .filter(|message_profile| message_profile.reply_id.is_none())
            .map(|message_profile| {
                html! {
                    <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                        auth_user=auth_user.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone()
                        message_edit_style=MessageEdit::Thread
                        on_reply_to=Some(on_reply_to.clone()) 
                        on_message_delete=on_message_delete.clone()
                        replying_to=None
                        mod_commets=false
                        stats_messages_reply=stats.clone() />
                }
            })
            .collect::<Html>();
        let class_modal_response = if self.replying_to.is_some() {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_reponse_scroll = if self.replying_to.is_some() {
            "display: block;"
        } else {
            "display: none;"
        };
        let maybe_modal_messages = html! {
            <div class=class_modal_response id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style=class_reponse_scroll aria-modal="true" role="dialog">
                <div class="modal-dialog modal-dialog-scrollable modal-xl">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="text-primary-blue-dark noir-bold is-size-36 lh-43 text-capitalize">{lang::dict("Comments")}</h1>
                            {maybe_exit_thread}
                        </div>
                        <div class="modal-body vh-100">
                            <div class="px-4 d-flex flex-column-reverse">
                                <div style="width: 100%; border-radius: 5px; margin-top: 15px;">{maybe_add_message}</div>
                                {messages}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        };
        let maybe_add = html! {
            <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                auth_user=auth_user.clone() 
                group_category=group_category.clone() 
                message_profile=None
                message_edit_style=MessageEdit::Thread
                on_reply_to=None 
                mod_commets=false
                replying_to=self.replying_to
                stats_messages_reply=stats.clone() />
        };
        html! {
            <>
                {maybe_modal_messages}
                {maybe_add}
                {list_messages}
            </>
        }
    }
}