use log::*;
use uuid::Uuid;
use yew::prelude::*;
use std::collections::HashMap;
use code_location::code_location;
use crate::message_card::MessageCard;
use crate::{MessageProfile, MessageAuthor};
use crate::{MessageGroupCategory, message_card::MessageEdit};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::message_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_types::types::{LessonId, PostId, MessageId, GroupId, UserId, RobotId, AppRoute, MyUserProfile};

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
    save_task: Option<RequestTask>,
    messages: Vec<MessageProfile>,
    replying_to: Option<MessageId>,   
    // content: String,
    // replies_private: bool,
    update_message: Option<message_model::update_message_content_by_id::ResponseData>,
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
    UpdateMessage(MessageId, String, bool),
    ContentProfileUpdate(Option<message_model::update_message_content_by_id::ResponseData>),
}

impl Component for MessageList {
    type Message = MessageListMessage;
    type Properties = MessageListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        match props.group_category {
            MessageGroupCategory::Lessons(group_id, lesson_id) => {
                link.send_message(MessageListMessage::FetchMessagesByLessonGroup(
                    lesson_id, group_id,
                ));
            }
            MessageGroupCategory::Posts(group_id, post_id) => link.send_message(
                MessageListMessage::FetchMessagesByPostGroup(post_id, group_id),
            ),
            MessageGroupCategory::Robots(group_id, robot_id) => link.send_message(
                MessageListMessage::FetchMessagesByRobotGroup(robot_id, group_id),
            ),
            MessageGroupCategory::DirectMessages(group_id) => link.send_message(
                MessageListMessage::FetchMessagesByDirectMessageGroup(group_id),
            ),
            MessageGroupCategory::FilesUser => {
                link.send_message(MessageListMessage::FetchMessagesWithFilesUser)
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
            save_task: None,
            update_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MessageListMessage::FetchMessagesByLessonGroup(lesson_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::messages_by_lesson_group::Variables {
                        group_id: group_id.0,
                        lesson_id: lesson_id.0,
                    };
                    let task = message_model::MessagesByLessonGroup::subscribe(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessageLessons(response)
                        }
                    );
                    self.lessons_sub = Some(task);
                }
            }
            MessageListMessage::MessageLessons(response) => {
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
            MessageListMessage::FetchMessagesByPostGroup(post_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::messages_by_post_group::Variables {
                        group_id: group_id.0,
                        post_id: post_id.0,
                    };
                    let task = message_model::MessagesByPostGroup::subscribe(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessagePosts(response)
                        }
                    );
                    self.posts_sub = Some(task);
                }
            }
            MessageListMessage::MessagePosts(response) => {
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
            MessageListMessage::FetchMessagesByDirectMessageGroup(group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::messages_by_direct_message_group::Variables {
                        group_id: group_id.0,
                    };
                    let task = message_model::MessagesByDirectMessageGroup::subscribe(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessagesDirect(response)
                        }
                    );
                    self.direct_sub = Some(task);
                }
            }
            MessageListMessage::FetchMessagesWithFilesUser => {
                let user_id = self.props.user_id.unwrap();
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::contribution_files_by_author_id::Variables {
                        author_id: user_id.0,
                        search: "%href%".to_string(),
                    };
                    let task = message_model::ContributionFilesByAuthorId::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::MessagesFiles(response)
                        }
                    );
                    self.task_messages = Some(task);
                }
            }
            MessageListMessage::MessagesDirect(response) => {
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
            MessageListMessage::MessagesFiles(response) => {
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
            MessageListMessage::UpdateMessage(message_id, content, replies_private) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::update_message_content_by_id::Variables {
                        message_id: message_id.0,
                        content,
                        replies_private,
                    };
                    let task = message_model::UpdateMessageContentById::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListMessage::ContentProfileUpdate(response)
                        }
                    );
                    self.save_task = Some(task);
                }
            }
            MessageListMessage::ContentProfileUpdate(response) => {
                self.update_message = response;
            }
        }

        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_category != props.group_category {
            match props.group_category {
                MessageGroupCategory::Lessons(group_id, lesson_id) => {
                    self.link.send_message(MessageListMessage::FetchMessagesByLessonGroup(
                        lesson_id, group_id,
                    ));
                }
                MessageGroupCategory::Posts(group_id, post_id) => self.link.send_message(
                    MessageListMessage::FetchMessagesByPostGroup(post_id, group_id),
                ),
                MessageGroupCategory::Robots(group_id, robot_id) => self.link.send_message(
                    MessageListMessage::FetchMessagesByRobotGroup(robot_id, group_id),
                ),
                MessageGroupCategory::DirectMessages(group_id) => self.link.send_message(
                    MessageListMessage::FetchMessagesByDirectMessageGroup(group_id),
                ),
                MessageGroupCategory::FilesUser => {
                    self.link.send_message(MessageListMessage::FetchMessagesWithFilesUser)
                }
            }
        }
        if self.props != props {
            self.props = props;
            should_render= true;
        }
        
        should_render
    }

    fn view(&self) -> Html {
        let on_message_delete = self.link.callback(|message_id| MessageListMessage::DeleteMessage(message_id));
        let on_change_list = self.link.callback(|(message_id, content, replies_private)| MessageListMessage::UpdateMessage(message_id, content, replies_private));

        let user_profile = self.props.user_profile.clone();
        let group_category = self.props.group_category;

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
                    <MessageCard on_app_route=self.props.on_app_route.clone() 
                        user_profile=user_profile.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone() 
                        on_message_delete=on_message_delete.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_change_list=on_change_list.clone()
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
                    <MessageCard on_app_route=self.props.on_app_route.clone() 
                        user_profile=user_profile.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_reply_to=Some(on_reply_to.clone()) 
                        replying_to=self.replying_to 
                        on_message_delete=on_message_delete.clone()
                        on_change_list=on_change_list.clone()
                        mod_commets=true
                        stats_messages_reply=stats.clone() />
                }
            })
            .collect::<Html>()
        };
        let maybe_add_message = html! {
            <MessageCard on_app_route=self.props.on_app_route.clone() 
                user_profile=user_profile.clone() 
                group_category=group_category.clone()
                message_profile=None 
                message_edit_style=MessageEdit::Thread
                on_change_list=on_change_list.clone()
                on_reply_to=None 
                mod_commets=false
                on_message_delete=on_message_delete.clone()
                replying_to=self.replying_to
                stats_messages_reply=stats.clone() />
        };
        let maybe_exit_thread = if self.replying_to.is_some() {
            let on_exit_thread = self.link.callback(move |_| MessageListMessage::ExitThread);
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
                    <MessageCard on_app_route=self.props.on_app_route.clone() 
                        user_profile=user_profile.clone() 
                        group_category=group_category.clone() 
                        message_profile=message_profile.clone()
                        message_edit_style=MessageEdit::Thread
                        on_reply_to=Some(on_reply_to.clone()) 
                        on_message_delete=on_message_delete.clone()
                        on_change_list=on_change_list.clone()
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
            <MessageCard on_app_route=self.props.on_app_route.clone() 
                user_profile=user_profile.clone() 
                group_category=group_category.clone() 
                message_profile=None
                message_edit_style=MessageEdit::Thread
                on_change_list=on_change_list.clone()
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