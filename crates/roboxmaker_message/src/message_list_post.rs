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
use roboxmaker_models::{user_model, message_model};
use roboxmaker_types::types::{PostId, MessageId, GroupId, UserId, AppRoute};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};


pub struct MessageListPost {
    link: ComponentLink<Self>,
    props: MessageListPostProperties,
    graphql_task: Option<GraphQLTask>,
    posts_sub: Option<SubscriptionTask>,
    delete_task: Option<RequestTask>,
    save_task: Option<RequestTask>,
    messages: Vec<MessageProfile>,
    replying_to: Option<MessageId>,  
    update_message: Option<message_model::update_message_content_by_id::ResponseData>,
    content: String,
    replies_private: bool,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct MessageListPostProperties {
    pub on_app_route: Option<Callback<AppRoute>>,
    pub user_id: Option<UserId>,
    pub user_profile: Option<MyUserProfile>,,
    pub group_id: GroupId,
    pub post_id: PostId,   
}

#[derive(Debug)]
pub enum MessageListPostMessage {
    FetchMessagesByPostGroup,
    MessagePosts(Option<message_model::messages_by_post_group::ResponseData>),
    ReplyTo(MessageId),
    ExitThread,
    DeleteMessage(MessageId),
    MessageDeleted(Option<message_model::delete_message_by_id::ResponseData>),
    UpdateMessage(MessageId, String, bool),
    ContentProfileUpdate(Option<message_model::update_message_content_by_id::ResponseData>),
    Send,
    MessagePostsCreated(Option<message_model::message_post_group_create::ResponseData>),
    OnPrivateRepliesToggle,
    OnContent(String),
}

impl Component for MessageListPost {
    type Message = MessageListPostMessage;
    type Properties = MessageListPostProperties;

    fn create(ctx: &Context<Self>) -> Self {
        link().send_message(MessageListPostMessage::FetchMessagesByPostGroup);
        MessageListPost {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            posts_sub: None,
            messages: vec![],
            replying_to: None, 
            delete_task: None,
            save_task: None,
            update_message: None,
            content: String::from(""),
            replies_private: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MessageListPostMessage::FetchMessagesByPostGroup => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::messages_by_post_group::Variables {
                        group_id: ctx.props().group_id.0,
                        post_id: ctx.props().post_id.0,
                    };
                    let task = message_model::MessagesByPostGroup::subscribe(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListPostMessage::MessagePosts(response)
                        }
                    );
                    self.posts_sub = Some(task);
                }
            }
            MessageListPostMessage::MessagePosts(response) => {
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
            MessageListPostMessage::ReplyTo(message_id) => {
                self.replying_to = Some(message_id);
            }
            MessageListPostMessage::ExitThread => {
                self.replying_to = None;
            }
            MessageListPostMessage::DeleteMessage(message_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::delete_message_by_id::Variables {
                        message_id: message_id.0,
                    };
                    let task = message_model::DeleteMessageById::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListPostMessage::MessageDeleted(response)
                        }
                    );
                    self.delete_task = Some(task);
                }
            }
            MessageListPostMessage::MessageDeleted(_) => {}
            MessageListPostMessage::UpdateMessage(message_id, content, replies_private) => {
                self.content = content.clone();
                self.replies_private = replies_private;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::update_message_content_by_id::Variables {
                        message_id: message_id.0,
                        content: content.clone(),
                        replies_private,
                    };
                    let task = message_model::UpdateMessageContentById::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListPostMessage::ContentProfileUpdate(response)
                        }
                    );
                    self.save_task = Some(task);
                }
            }
            MessageListPostMessage::ContentProfileUpdate(response) => {
                self.update_message = response;
            }
            MessageListPostMessage::Send => {
                let local = chrono::Local::now().naive_local();
                let content = self.content.clone();
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::message_post_group_create::Variables {
                        content,
                        group_id: ctx.props().group_id.0,
                        post_id: ctx.props().post_id.0,
                        reply_id: self.replying_to.and_then(|reply_id| Some(reply_id.0)),
                        replies_private: self.replies_private,
                        timestamp: local,
                    };
                    let task = message_model::MessagePostGroupCreate::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessageListPostMessage::MessagePostsCreated(response)
                        }
                    );
                    self.save_task = Some(task);
                }
            }
            MessageListPostMessage::MessagePostsCreated(_) => {}
            MessageListPostMessage::OnPrivateRepliesToggle => {
                if self.replies_private {
                    self.replies_private = false;
                } else {
                    self.replies_private = true;
                }
            }
            MessageListPostMessage::OnContent(content) => {
                self.content = content;
            }
        }

        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            self.link().send_message(MessageListPostMessage::FetchMessagesByPostGroup);
            should_render= true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let auth_user = ctx.props().auth_user.clone();
        let on_message_delete = ctx.link().callback(|message_id| MessageListPostMessage::DeleteMessage(message_id));
        let on_change_list = ctx.link().callback(|(message_id, content, replies_private)| MessageListPostMessage::UpdateMessage(message_id, content, replies_private));

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
            .callback(move |message_id| MessageListPostMessage::ReplyTo(message_id));

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
                        group_category=MessageGroupCategory::Posts(ctx.props().group_id, ctx.props().post_id)
                        message_profile=message_profile.clone() 
                        on_message_delete=on_message_delete.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_reply_to=None
                        on_change_list=on_change_list.clone()
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
                        group_category=MessageGroupCategory::Posts(ctx.props().group_id, ctx.props().post_id)
                        message_profile=message_profile.clone()
                        on_message_delete=on_message_delete.clone()
                        message_edit_style=MessageEdit::EditModal
                        on_reply_to=Some(on_reply_to.clone()) 
                        on_change_list=on_change_list.clone()
                        replying_to=self.replying_to 
                        mod_commets=true
                        stats_messages_reply=stats.clone() />
                }
            })
            .collect::<Html>()
        };
        let maybe_add_message = html! {
            <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                auth_user=auth_user.clone() 
                group_category=MessageGroupCategory::Posts(ctx.props().group_id, ctx.props().post_id)
                message_profile=None 
                message_edit_style=MessageEdit::Thread
                on_message_delete=on_message_delete.clone()
                on_change_list=on_change_list.clone()
                on_reply_to=None 
                mod_commets=false
                replying_to=self.replying_to
                stats_messages_reply=stats.clone() />
        };
        let maybe_exit_thread = if self.replying_to.is_some() {
            let on_exit_thread = ctx.link().callback(move |_| MessageListPostMessage::ExitThread);
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
            .callback(move |message_id| MessageListPostMessage::ReplyTo(message_id));
        let list_messages = self
            .messages
            .iter()
            .filter(|message_profile| message_profile.reply_id.is_none())
            .map(|message_profile| {
                html! {
                    <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                        auth_user=auth_user.clone() 
                        group_category=MessageGroupCategory::Posts(ctx.props().group_id, ctx.props().post_id)
                        message_profile=message_profile.clone()
                        on_message_delete=on_message_delete.clone()
                        message_edit_style=MessageEdit::Thread
                        on_reply_to=Some(on_reply_to.clone()) 
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
        // let on_replies_private_toggle = self
        //     .link
        //     .callback(|_: MouseEvent| MessageListPostMessage::OnPrivateRepliesToggle);
        // let maybe_replies = ctx.props().auth_user.clone().and_then(|data| data.user_by_pk).clone().and_then(|user| {
        //     if user.user_teacher.is_some() || user.user_staff.is_some() && self.replying_to.is_none() {
        //         Some(html! {
        //             <label class="checkbox d-flex align-items-center">
        //                 <input type="checkbox" checked=self.replies_private onclick=&on_replies_private_toggle />
        //                 <span class="ps-3">{lang::dict("Private replies?")}</span>
        //             </label>
        //         })
        //     } else {
        //         None
        //     }
        // }).unwrap_or(html! {});
        // let on_data = self
        //     .link
        //     .callback(move |data| MessageListPostMessage::OnContent(data));

        // let on_send = ctx.link().callback(move |_| MessageListPostMessage::Send);

        // let on_replies_private_toggle = self
        //     .link
        //     .callback(|_: MouseEvent| MessageListPostMessage::OnPrivateRepliesToggle);

        // let maybe_replies_private = if self.content.len() > 0 {
        //     maybe_replies
        // } else {
        //     html! {}
        // };
        // let maybe_replies_student = ctx.props().auth_user.clone().and_then(|data| data.user_by_pk).clone().and_then(|user| {
        //     if user.user_student.is_some() && self.replying_to.is_some() {
        //         Some(html! {
        //             <label class="checkbox d-flex align-items-center">
        //                 <input type="checkbox" checked=self.replies_private onclick=&on_replies_private_toggle />
        //                 <span class="ps-3">{lang::dict("Private replies?")}</span>
        //             </label>
        //         })
        //     } else {
        //         None
        //     }
        // }).unwrap_or(html! {});
        // let maybe_replies_private_student = if self.content.len() > 0 {
        //     maybe_replies_student
        // } else {
        //     html! {}
        // };
        // let maybe_add = if self.content.len() > 1 {
        //     html! {
        //         <button class="send-message-to-space" onclick=&on_send>
        //             <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
        //         </button>
        //     }
        // } else {
        //     html! {
        //         <button class="send-message-to-space" disabled=true onclick=&on_send>
        //             <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
        //         </button>
        //     }
        // };
        let maybe_add = html! {
            <MessageCard on_app_route=ctx.props().on_app_route.clone() 
                auth_user=auth_user.clone() 
                group_category=MessageGroupCategory::Posts(ctx.props().group_id, ctx.props().post_id)
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