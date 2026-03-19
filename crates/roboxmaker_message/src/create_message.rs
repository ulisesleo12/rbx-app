use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use std::collections::HashMap;
use yew::web_sys::{Node, self};
use crate::MessageGroupCategory;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::{user_model, message_model};
use roboxmaker_types::types::{MessageId, AppRoute};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageEdit {
    Thread,
    EditModal,
}

pub struct MessageCard {
    link: ComponentLink<Self>,
    props: MessageCardProperties,
    graphql_task: Option<GraphQLTask>,
    save_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    node: Option<Node>,
    content: String,
    edit: bool,
    replies_private: bool,
    maybe_messages: bool,
    update_message: Option<message_model::update_message_content_by_id::ResponseData>,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct MessageCardProperties {
    pub on_app_route: Option<Callback<AppRoute>>,
    pub user_profile: Option<MyUserProfile>,,
    pub on_reply_to: Option<Callback<MessageId>>,
    pub replying_to: Option<MessageId>,
    pub mod_commets: bool,
}

#[derive(Debug)]
pub enum MessageContentSaved {
    Lesson(Option<message_model::message_lesson_group_create::ResponseData>),
    Post(Option<message_model::message_post_group_create::ResponseData>),
    Robot(Option<message_model::message_robot_group_create::ResponseData>),
    DirectMessage(Option<message_model::direct_message_group_create::ResponseData>),
}

#[derive(Debug)]
pub enum MessageCardMessage {
    OnContent(String),
    Send,
    OnPrivateRepliesToggle,
}

impl Component for MessageCard {
    type Message = MessageCardMessage;
    type Properties = MessageCardProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(MessageCardMessage::ContentUpdate);
        MessageCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            save_task: None,
            delete_task: None,
            node: None,
            content: String::from(""),
            edit: false,
            replies_private: false,
            maybe_messages: false,
            update_message: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);

        let should_update = true;
        match msg {
            MessageCardMessage::OnContent(content) => {
                self.content = content;
            }
            MessageCardMessage::Send => match self.props.group_category {
                MessageGroupCategory::Lessons(group_id, lesson_id) => {
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_lesson_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            lesson_id: lesson_id.0,
                            reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                        };
                        let task = message_model::MessageLessonGroupCreate::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Lesson(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::Posts(group_id, post_id) => {
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_post_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            post_id: post_id.0,
                            reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                        };
                        let task = message_model::MessagePostGroupCreate::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Post(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::Robots(group_id, robot_id) => {
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_robot_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            robot_id: robot_id.0,
                            reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                        };
                        let task = message_model::MessageRobotGroupCreate::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Robot(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::DirectMessages(group_id) => {
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::direct_message_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                        };
                        let task = message_model::DirectMessageGroupCreate::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::DirectMessage(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                _ => {}
            },
            MessageCardMessage::OnPrivateRepliesToggle => {
                if self.replies_private {
                    self.replies_private = false;
                } else {
                    self.replies_private = true;
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = true;

        if self.props != props {
            self.props = props;
            self.link.send_message(MessageCardMessage::ContentUpdate);
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);

        let message_class = if self.props.replying_to.is_some()
            && self
                .props
                .message_profile
                .as_ref()
                .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                != self.props.replying_to
        {
            "message-frame reply-to has-background-light p-2 ms-6 mb-3"
        } else {
            "message-frame mb-3"
        };

        if let Some(auth_user) = &self.props.auth_user {
            let on_replies_private_toggle = self
                .link
                .callback(|_: MouseEvent| MessageCardMessage::OnPrivateRepliesToggle);
            let maybe_replies = auth_user.user_by_pk.clone().and_then(|user| {
                if user.user_teacher.is_some() || user.user_staff.is_some() && self.props.replying_to.is_none() {
                    Some(html! {
                        <label class="checkbox d-flex align-items-center">
                            <input type="checkbox" checked=self.replies_private onclick=&on_replies_private_toggle />
                            <span class="ps-3">{lang::dict("Private replies?")}</span>
                        </label>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});
            let on_data = self
                .link
                .callback(move |data| MessageCardMessage::OnContent(data));

            let on_send = self.link.callback(move |_| MessageCardMessage::Send);

            let on_replies_private_toggle = self
                .link
                .callback(|_: MouseEvent| MessageCardMessage::OnPrivateRepliesToggle);

            let maybe_replies_private = if self.content.len() > 0 {
                maybe_replies
            } else {
                html! {}
            };
            let maybe_replies_student = auth_user.user_by_pk.clone().and_then(|user| {
                if user.user_student.is_some() && self.props.replying_to.is_some() {
                    Some(html! {
                        <label class="checkbox d-flex align-items-center">
                            <input type="checkbox" checked=self.replies_private onclick=&on_replies_private_toggle />
                            <span class="ps-3">{lang::dict("Private replies?")}</span>
                        </label>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});
            let maybe_replies_private_student = if self.content.len() > 0 {
                maybe_replies_student
            } else {
                html! {}
            };
            let maybe_add = if self.content.len() > 1 {
                html! {
                    <button class="send-message-to-space opacity-75" onclick=&on_send>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
                    </button>
                }
            } else {
                html! {
                    <button class="send-message-to-space" disabled=true onclick=&on_send>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
                    </button>
                }
            };
            let maybe_add_direct = if self.content.len() > 1 {
                html! {
                    <button class="send-message-to-space" onclick=&on_send>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Place")}</span>
                    </button>
                }
            } else {
                html! {
                    <button class="send-message-to-space opacity-75" disabled=true onclick=&on_send>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Place")}</span>
                    </button>
                }
            };
            let message_class_create = if self.props.replying_to.is_some()
                && self
                    .props
                    .message_profile
                    .as_ref()
                    .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                    != self.props.replying_to
            {
                "box-message-to-post bg-white"
            } else {
                "box-message-to-post-2 bg-white"
            };
            let maybe_created = html! {
                <>
                    <div class=message_class_create>
                        <ckeditor::CKEditor auth_user=self.props.auth_user.clone()
                            content=self.content.clone()
                            upload_url=upload_url.clone()
                            on_data=on_data.clone() />
                    </div>
                    <div class="d-flex justify-content-between align-items-center pt-4 pb-5">
                        <div>{maybe_replies_private.clone()}</div>
                        <div>{maybe_replies_private_student.clone()}</div>
                        <div>{ maybe_add }</div>
                    </div>
                </>
            };
            let maybe_created_2 = html! {
                <>
                    <div class="box-message-to-post bg-white">
                        <ckeditor::CKEditor auth_user=self.props.auth_user.clone()
                            content=self.content.clone()
                            upload_url=upload_url.clone()
                            on_data=on_data.clone() />
                    </div>
                    <div class="d-flex justify-content-between align-items-center pt-4 pb-5">
                        <div>{maybe_replies_private.clone()}</div>
                        <div>{ maybe_add_direct.clone() }</div>
                    </div>
                </>
            };
            let maybe_button_message = {
                match self.props.group_category {
                    MessageGroupCategory::Lessons(_group_id, _lesson_id) => {
                        html! {
                            {maybe_created.clone()}
                        }
                    }
                    MessageGroupCategory::Posts(_group_id, _post_id) => {
                        html! {
                            {maybe_created.clone()}
                        }
                        
                    }
                    MessageGroupCategory::Robots(_group_id, _robot_id) => {
                        html! {
                            {maybe_created.clone()}
                        }
                        
                    }
                    MessageGroupCategory::DirectMessages(_group_id) => {
                        html! {
                            {maybe_created_2}
                        }
                    }
                    MessageGroupCategory::FilesUser => {
                        html! {}
                    }
                }
            }; 
            html! {
                <>
                    {maybe_button_message}
                </>
            }
        } else {
            html! {}
        }
    }
}
