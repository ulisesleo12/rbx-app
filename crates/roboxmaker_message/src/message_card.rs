use log::*;
use uuid::Uuid;
use yew::prelude::*;
use crate::MessageProfile;
use yew::virtual_dom::VNode;
use std::collections::HashMap;
use web_sys::{Node, self};
use crate::MessageGroupCategory;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::message_model;
use roboxmaker_types::types::{MessageId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageEdit {
    Thread,
    EditModal,
}

pub struct MessageCard {
    graphql_task: Option<GraphQLTask>,
    save_task: Option<RequestTask>,
    node: Option<Node>,
    content: String,
    edit: bool,
    replies_private: bool,
    maybe_messages: bool,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct MessageCardProperties {
    pub user_profile: Option<MyUserProfile>,
    pub group_category: MessageGroupCategory,
    pub message_profile: Option<MessageProfile>,
    pub on_reply_to: Option<Callback<MessageId>>,
    pub replying_to: Option<MessageId>,
    pub stats_messages_reply: HashMap<Uuid, usize>,
    pub message_edit_style: MessageEdit,
    pub mod_commets: bool,
    pub on_message_delete: Option<Callback<MessageId>>,
    pub on_change_list: Callback<(MessageId, String, bool)>,
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
    ContentUpdate,
    OnContent(String),
    Send,
    ContentSaved(MessageContentSaved),
    EditMessage(MessageId),
    CancelEditMessage(MessageId),
    OnReplyTo(MessageId),
    OnPrivateRepliesToggle,
    DeleteMessage(MessageId),
    MessageUpdate(MessageId),
}

impl Component for MessageCard {
    type Message = MessageCardMessage;
    type Properties = MessageCardProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MessageCardMessage::ContentUpdate);
        MessageCard {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            save_task: None,
            node: None,
            content: String::from(""),
            edit: false,
            replies_private: false,
            maybe_messages: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);

        let should_update = true;
        match msg {
            MessageCardMessage::ContentUpdate => {
                self.replies_private = ctx.props().message_profile.clone().and_then(|data| Some(data.replies_private)).unwrap_or(false);
                self.content = ctx.props().message_profile.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string());
                self.node = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.create_element("div").ok())
                    .and_then(|div| {
                        div.set_class_name("ck-content");
                        div.set_inner_html(&self.content);
                        Some(Node::from(div))
                    });
            }
            MessageCardMessage::OnContent(content) => {
                self.content = content;
            }
            MessageCardMessage::ContentSaved(_) => {
                self.content = String::default();
            }
            MessageCardMessage::Send => match ctx.props().group_category {
                MessageGroupCategory::Lessons(group_id, lesson_id) => {
                    let local = chrono::Local::now().naive_local();
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_lesson_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            lesson_id: lesson_id.0,
                            reply_id: ctx.props().replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                            timestamp: local,
                        };
                        let task = message_model::MessageLessonGroupCreate::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Lesson(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::Posts(group_id, post_id) => {
                    let local = chrono::Local::now().naive_local();
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_post_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            post_id: post_id.0,
                            reply_id: ctx.props().replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                            timestamp: local,
                        };
                        let task = message_model::MessagePostGroupCreate::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Post(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::Robots(group_id, robot_id) => {
                    let local = chrono::Local::now().naive_local();
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::message_robot_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            robot_id: robot_id.0,
                            reply_id: ctx.props().replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                            timestamp: local,
                        };
                        let task = message_model::MessageRobotGroupCreate::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                MessageCardMessage::ContentSaved(MessageContentSaved::Robot(response))
                            }
                        );
                        self.save_task = Some(task);
                    }
                }
                MessageGroupCategory::DirectMessages(group_id) => {
                    let local = chrono::Local::now().naive_local();
                    let content = self.content.clone();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = message_model::direct_message_group_create::Variables {
                            content,
                            group_id: group_id.0,
                            reply_id: ctx.props().replying_to.and_then(|reply_id| Some(reply_id.0)),
                            replies_private: self.replies_private,
                            timestamp: local,
                        };
                        let task = message_model::DirectMessageGroupCreate::request(
                            graphql_task, 
                            &ctx, 
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
            MessageCardMessage::EditMessage(_message_id) => {
                self.edit = true;
                self.maybe_messages = true;
            }
            MessageCardMessage::CancelEditMessage(_message_id) => {
                self.edit = false;
                self.maybe_messages = false;
            }
            MessageCardMessage::OnReplyTo(message_id) => {
                ctx.props()
                    .on_reply_to
                    .as_ref()
                    .and_then(|on_reply_to| Some(on_reply_to.emit(message_id)));
            }
            MessageCardMessage::OnPrivateRepliesToggle => {
                if self.replies_private {
                    self.replies_private = false;
                } else {
                    self.replies_private = true;
                }
            }
            MessageCardMessage::DeleteMessage(message_id) => {
                if let Some(on_message_delete) = &ctx.props().on_message_delete {
                    on_message_delete.emit(message_id)
                }
            }
            MessageCardMessage::MessageUpdate(message_id) => {
                ctx.props().on_change_list.emit((message_id, self.content.clone(), self.replies_private));
                self.edit = false;
                self.maybe_messages = false;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = true;

        if ctx.props() != old_props {
            // ctx.props() = old_props;
            ctx.link().send_message(MessageCardMessage::ContentUpdate);
            should_render = true;
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);

        let message_class = if ctx.props().replying_to.is_some()
            && ctx
                .props()
                .message_profile
                .as_ref()
                .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                != ctx.props().replying_to
        {
            "message-frame reply-to has-background-light p-2 ms-6 mb-3"
        } else {
            "message-frame mb-3"
        };

        if let Some(message_profile) = &ctx.props().message_profile {
            if self.edit {
                let message_id = MessageId(message_profile.message_id);

                let on_data = ctx
                    .link()
                    .callback(move |data| MessageCardMessage::OnContent(data));

                let on_save = ctx
                    .link()
                    .callback(move |_| MessageCardMessage::MessageUpdate(message_id));

                let on_cancel = ctx
                    .link()
                    .callback(move |_| MessageCardMessage::CancelEditMessage(message_id));

                let on_replies_private_toggle_edit = ctx
                    .link()
                    .callback(|_: MouseEvent| MessageCardMessage::OnPrivateRepliesToggle);

                let maybe_replies_private = html! {
                    <label class="checkbox">
                        <span class="pe-2">{lang::dict("Private replies?")}</span>
                        <input class="bg-checkbox" type="checkbox" checked={self.replies_private}
                            onclick={&on_replies_private_toggle_edit} />
                    </label>
                };
                let maybe_replies_private_edith = ctx.props().user_profile.as_ref().and_then(|user| {
                    if user.user_staff.is_some() || user.user_teacher.is_some() {
                        Some(html! {
                            <label class="checkbox">
                                <span class="pe-2">{lang::dict("Private replies?")}</span>
                                <input class="bg-checkbox" type="checkbox" checked={self.replies_private}
                                    onclick={&on_replies_private_toggle_edit} />
                            </label>
                        })
                    } else {
                        Some(html! {})
                    }
                }).unwrap_or(html! {});
                let class_modal_response = if self.maybe_messages {
                    "modal fade show mb-4"
                } else {
                    "modal fade"
                };
                let class_reponse_scroll = if self.maybe_messages {
                    "display: block;"
                } else {
                    "display: none;"
                };
                let maybe_edit_option = {
                    match ctx.props().message_edit_style {
                        MessageEdit::Thread => {
                            html! {
                                <div class={class_modal_response} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_reponse_scroll} aria-modal="true" role="dialog">
                                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                                        <div class="modal-content">
                                            <div class="modal-header">
                                                <h1 class="hello-my-space text-center">{lang::dict("Edit")}</h1>
                                                <a class="btn bg-purple-on ms-5" onclick={&on_cancel}>
                                                    <span class="text-white">
                                                        <i class="fas fa-times"></i>
                                                    </span>
                                                </a>
                                            </div>
                                            <div class="modal-body vh-80">
                                                <div class="px-4">
                                                    <div class="box-my-space-direct bg-white d-flex flex-column">
                                                        <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                                                            content={self.content.clone()} upload_url={upload_url.clone()}
                                                            on_data={on_data.clone()} />
                                                        <div class="d-flex justify-content-end pe-4">
                                                            {maybe_replies_private_edith}</div>
                                                    </div>
                                                    <div class="d-flex flex-wrap align-items-center justify-content-evenly mt-3">
                                                        <button class="btn btn-purple-on border-0 rounded-3"
                                                            onclick={&on_cancel}>
                                                            <span class="text-white">
                                                                <span class="me-2">
                                                                    <i class="fas fa-times fas fal-lg"></i>
                                                                </span>
                                                                <span>{lang::dict("Cancel")}</span>
                                                            </span>
                                                        </button>
                                                        <button class="btn btn-primary-blue-dark border-0 rounded-3"
                                                            onclick={on_save.clone()}>
                                                            <span class="text-white">
                                                                <span class="me-2">
                                                                    <i class="fas fa-check"></i>
                                                                </span>
                                                                <span>{lang::dict("Save")}</span>
                                                            </span>
                                                        </button>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        },
                        MessageEdit::EditModal => {
                            html! {
                                <div key={message_id.to_string()}>
                                    <div class="box-my-space-direct bg-white d-flex flex-column">
                                        <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                                            content={self.content.clone()}
                                            upload_url={upload_url.clone()} on_data={on_data.clone()} />
                                        <div class="d-flex justify-content-end pe-4">
                                            {maybe_replies_private}</div>
                                    </div>
                                    <div class="d-flex flex-wrap align-items-center justify-content-evenly mt-3">
                                        <button class="btn btn-purple-on border-0 rounded-3" onclick={on_cancel.clone()}>
                                            <span class="text-white">
                                                <span class="me-2">
                                                    <i class="fas fa-times fas fal-lg"></i>
                                                </span>
                                                <span>{lang::dict("Cancel")}</span>
                                            </span>
                                        </button>
                                        <button class="btn btn-primary-blue-dark border-0 rounded-3" onclick={on_save.clone()}>
                                            <span class="text-white">
                                                <span class="me-2">
                                                    <i class="fas fa-check"></i>
                                                </span>
                                                <span>{lang::dict("Save")}</span>
                                            </span>
                                        </button>
                                    </div>
                                </div>
                            }
                        },
                    }
                };
                let maybe_style_message = {
                    match ctx.props().group_category {
                        MessageGroupCategory::Lessons(_group_id, _lesson_id) => {
                            html! {
                                {maybe_edit_option.clone()}
                            }
                        }
                        MessageGroupCategory::Posts(_group_id, _post_id) => {
                            html! {
                                {maybe_edit_option.clone()}
                            }
                            
                        }
                        MessageGroupCategory::Robots(_group_id, _robot_id) => {
                            html! {
                                {maybe_edit_option.clone()}
                            }
                            
                        }
                        MessageGroupCategory::DirectMessages(_group_id) => {
                            html! {
                                <div class={class_modal_response} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_reponse_scroll} aria-modal="true" role="dialog">
                                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                                        <div class="modal-content">
                                            <div class="modal-header">
                                                <h1 class="hello-my-space text-center">{lang::dict("Edit")}</h1>
                                                <a class="btn bg-purple-on ms-5" onclick={&on_cancel}>
                                                    <span class="text-white">
                                                        <i class="fas fa-times"></i>
                                                    </span>
                                                </a>
                                            </div>
                                            <div class="modal-body vh-80">
                                                <div class="px-4">
                                                    <div key={message_id.to_string()}>
                                                        <div class="box-my-space-direct bg-white d-flex flex-column">
                                                            <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                                                                content={self.content.clone()} upload_url={upload_url.clone()}
                                                                on_data={on_data.clone()} />
                                                        </div>
                                                        <div class="d-flex flex-wrap align-items-center justify-content-evenly mt-3">
                                                            <button class="btn btn-purple-on border-0 rounded-3"
                                                                onclick={&on_cancel}>
                                                                <span class="text-white">
                                                                    <span class="me-2">
                                                                        <i class="fas fa-times fas fal-lg"></i>
                                                                    </span>
                                                                    <span>{lang::dict("Cancel")}</span>
                                                                </span>
                                                            </button>
                                                            <button class="btn btn-primary-blue-dark border-0 rounded-3"
                                                                onclick={on_save.clone()}>
                                                                <span class="text-white">
                                                                    <span class="me-2">
                                                                        <i class="fas fa-check"></i>
                                                                    </span>
                                                                    <span>{lang::dict("Save")}</span>
                                                                </span>
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                        MessageGroupCategory::FilesUser => {
                            html! { }
                        }
                    }
                };
                html! {
                    {maybe_style_message}
                }
            } else {
                let maybe_node = if let Some(node) = &self.node {
                    VNode::VRef(node.clone())
                } else {
                    html! {}
                };

                let maybe_message_edit = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(user, message_profile)| {
                        let message_id = message_profile.message_id;
                        let author_id = message_profile.author.user_id;
                        let on_message_edit = ctx.link().callback(move |_| {
                            MessageCardMessage::EditMessage(MessageId(message_id))
                        });
                        if user.user_id.0 == author_id || user.user_staff.is_some() || user.user_teacher.is_some() && (ctx.props().replying_to.is_none()
                                || ctx.props().replying_to != Some(MessageId(message_id)))
                        {
                            Some(html! {
                                <button class="btn bg-white text-purple-gray me-4" onclick={on_message_edit}>
                                    <i class="far fa-edit fas fa-lg"></i>
                                </button>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});

                let maybe_message_edit_two = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(user, message_profile)| {
                        let message_id = message_profile.message_id;
                        let author_id = message_profile.author.user_id;
                        let on_message_edit = ctx.link().callback(move |_| {
                            MessageCardMessage::EditMessage(MessageId(message_id))
                        });
                        if (user.user_id.0 == author_id || user.user_staff.is_some() || 
                            ( user.user_teacher.is_some() && message_profile.author.user_student.is_some()))
                            && (ctx.props().replying_to.is_none() || ctx.props().replying_to != Some(MessageId(message_id))) {
                            Some(html! {
                                <button class="btn btn-outline-brown border-0 btn-sm" onclick={on_message_edit}>
                                    <i class="far fa-edit fas fa-lg"></i>
                                </button>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});

                let maybe_message_delete = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(user, message_profile)| {
                        let message_id = message_profile.message_id;
                        let author_id = message_profile.author.user_id;
                        let on_message_delete = ctx.link().callback(move |_| {
                            MessageCardMessage::DeleteMessage(MessageId(message_id))
                        });
                        if (user.user_id.0 == author_id || user.user_staff.is_some() || 
                            ( user.user_teacher.is_some() && message_profile.author.user_student.is_some()))
                            && (ctx.props().replying_to.is_none() || ctx.props().replying_to != Some(MessageId(message_id))) {
                            Some(html! {
                                <button class="btn bg-white text-purple-gray" onclick={on_message_delete}>
                                    <i class="far fa-trash-alt fas fa-lg"></i>
                                </button>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});
                let class_margin_btn = if ctx.props().replying_to.is_some()
                    && ctx
                        .props()
                        .message_profile
                        .as_ref()
                        .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                        != ctx.props().replying_to
                {
                    "btn btn-outline-brown border-0 btn-sm"
                } else {
                    "btn btn-outline-brown border-0 btn-sm mt-2"
                };
                let maybe_message_delete_two = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(item, message_profile)| {
                        let message_id = message_profile.message_id;
                        let author_id = message_profile.author.user_id;
                        let on_message_delete = ctx.link().callback(move |_| {
                            MessageCardMessage::DeleteMessage(MessageId(message_id))
                        });
                        if (item.user_id.0 == author_id || item.user_staff.is_some() || 
                            ( item.user_teacher.is_some() && message_profile.author.user_student.is_some()))
                            && (ctx.props().replying_to.is_none() || ctx.props().replying_to != Some(MessageId(message_id))) {
                            Some(html! {
                                <button class={class_margin_btn} onclick={on_message_delete}>
                                    <i class="far fa-trash-alt fas fa-lg"></i>
                                </button>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});

                let maybe_message_delete_files = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(item, message_profile)| {
                        let message_id = message_profile.message_id;
                        let author_id = message_profile.author.user_id;
                        let on_message_delete = ctx.link().callback(move |_| {
                            MessageCardMessage::DeleteMessage(MessageId(message_id))
                        });
                        if (item.user_id.0 == author_id || item.user_staff.is_some() || 
                            ( item.user_teacher.is_some() && message_profile.author.user_student.is_some()))
                            && (ctx.props().replying_to.is_none() || ctx.props().replying_to != Some(MessageId(message_id))) {
                            Some(html! {
                                <button class="btn bg-white text-purple-gray" onclick={on_message_delete}>
                                    <i class="far fa-trash-alt fas fa-lg"></i>
                                </button>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});
                let maybe_message_reply_to = ctx
                    .props()
                    .on_reply_to
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(_on_reply_to, message_profile)| {
                        let message_id = message_profile.message_id;
                        let on_reply_to = ctx.link().callback(move |_| {
                            MessageCardMessage::OnReplyTo(MessageId(message_id))
                        });
                        Some(html! {
                            <a onclick={on_reply_to}>
                                <span class="text-brown noir-light is-size-14 lh-17 ps-2">{"Ver "}<span>{format!("{}",
                                    ctx.props().stats_messages_reply.get(&message_id).unwrap_or(&0))}</span>{" respuestas"}</span>
                            </a>
                        })
                    })
                    .unwrap_or_default();

                let maybe_message_reply_message = ctx
                    .props()
                    .on_reply_to
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(_on_reply_to, message_profile)| {
                        let message_id = message_profile.message_id;
                        let on_reply_to = ctx.link().callback(move |_| {
                            MessageCardMessage::OnReplyTo(MessageId(message_id))
                        });
                        Some(html! {
                            <a onclick={on_reply_to}>
                                <span class="text-secondary-purple noir-medium is-size-14 lh-17 ps-3">{"Responder"}</span>
                            </a>
                        })
                    })
                    .unwrap_or_default();

                let message_div_key = ctx
                    .props()
                    .message_profile
                    .as_ref()
                    .and_then(|message_profile| Some(message_profile.message_id))
                    .unwrap_or_default();

                let is_replies_private = if self.replies_private {
                    html! {
                        <>
                            <span class="icon is-small pt-2">
                                <i class="fas fa-lock"></i>
                            </span>
                        </>
                    }
                } else {
                    html! {}
                };
                let class_module_txt = if ctx.props().replying_to.is_some()
                    && ctx
                        .props()
                        .message_profile
                        .as_ref()
                        .and_then(|message_profile| Some(MessageId(message_profile.message_id))).is_some()
                        // != ctx.props().replying_to
                {
                    "module-message line-clamp-message text-brown noir-light is-size-16 lh-19 text-justify"
                } else {
                    "module-message line-clamp-message text-brown noir-light is-size-16 lh-19 text-justify"
                };

                let maybe_author_message = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .zip(ctx.props().message_profile.as_ref())
                    .and_then(|(user, message_profile)| {
                        let author_id = message_profile.author.user_id;
                        if user.user_id.0 == author_id {
                            Some(html! {
                                <span class="me-2">{"(Tú)"}</span>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});
                let pic_path_class_message = if ctx.props().replying_to.is_some()
                    && ctx
                        .props()
                        .message_profile
                        .as_ref()
                        .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                        != ctx.props().replying_to
                {
                    "img-card-32"
                } else {
                    "img-card-48"
                };
                let class_author_class = if ctx.props().replying_to.is_some()
                    && ctx
                        .props()
                        .message_profile
                        .as_ref()
                        .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                        != ctx.props().replying_to
                {
                    "text-primary-blue-dark noir-bold is-size-16 lh-25"
                } else {
                    "text-primary-blue-dark noir-bold is-size-16 lh-25"
                };
                let class_options_message = if ctx.props().mod_commets {
                    "d-flex flex-wrap h-20"
                } else {
                    "d-flex flex-column"
                };
                let class_display_mod = if ctx.props().mod_commets {
                    "d-flex flex-row"
                } else {
                    "d-flex flex-row justify-content-between"
                };
                let message_card_one = html! {
                    <div key={message_div_key.to_string()} class={message_class}>
                        <div class={class_display_mod}>
                            <img src={message_profile.author.pic_path.clone()} class={pic_path_class_message} />
                            <div class="d-flex justify-content-start flex-column ms-2 w-80">
                                <span class={class_author_class}>{&message_profile.author.full_name}<span class="mx-2">{is_replies_private}</span>{maybe_author_message}</span>
                                <span class={class_module_txt}>
                                    { maybe_node.clone() }
                                </span>
                                <div class="d-flex flex-wrap align-items-center">
                                    <span class="text-secondary-purple noir-medium is-size-14 lh-17">{&message_profile.timestamp}</span>
                                    {maybe_message_reply_message}
                                </div>
                                <div class="d-flex flex-wrap align-items-center">
                                    <hr style="width: 30%; border: 1px solid #615967; margin-top: 14px; margin-bottom: 14px; " />
                                    {maybe_message_reply_to}
                                </div>
                            </div>
                            <div class={class_options_message}>
                                {maybe_message_edit_two}
                                {maybe_message_delete_two}
                            </div>
                        </div>
                    </div>
                };
                let message_card_direct = html! {
                    <div key={message_div_key.to_string()} class={message_class}>
                        <div class="card-messages-view-direct bg-white mb-4">
                            <div class="d-flex flex-column p-4">
                                <span class="text-primary-blue-dark noir-light is-size-16 lh-19 text-justify">
                                    { maybe_node.clone() }
                                </span>
                                <div class="d-flex align-items-center justify-content-end pt-5 mt-2">
                                    {maybe_message_edit}
                                    {maybe_message_delete}
                                </div>
                            </div>
                        </div>
                    </div>
                };
                let maybe_style_message = {
                    match ctx.props().group_category {
                        MessageGroupCategory::Lessons(_group_id, _lesson_id) => {
                            html! {
                                {message_card_one.clone()}
                            }
                        }
                        MessageGroupCategory::Posts(_group_id, _post_id) => {
                            html! {
                                {message_card_one.clone()}
                            }
                            
                        }
                        MessageGroupCategory::Robots(_group_id, _robot_id) => {
                            html! {
                                {message_card_one.clone()}
                            }
                            
                        }
                        MessageGroupCategory::DirectMessages(_group_id) => {
                            html! {
                                {message_card_direct}
                            }
                        }
                        MessageGroupCategory::FilesUser => {
                            html! {
                                <div key={message_div_key.to_string()} class={message_class}>
                                    <div class="card-messages-view-files-user bg-white d-flex align-items-center justify-content-between mb-4">
                                        <span class="d-flex align-items-center text-primary-blue-dark noir-bold is-size-16 lh-22">
                                            <span class="me-3">
                                                <i class="fas fa-paperclip"></i>
                                            </span>
                                            { maybe_node.clone() }
                                        </span>
                                        <div
                                            class="d-flex align-items-center">
                                                <span class="text-brown noir-light is-size-16 lh-19 pe-4">{&message_profile.timestamp}</span>
                                            {maybe_message_delete_files}
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    }
                };  
                html! {
                    {maybe_style_message}
                }
            }
        } else if let Some(item) = &ctx.props().user_profile {
            let on_replies_private_toggle = ctx
                .link()
                .callback(|_: MouseEvent| MessageCardMessage::OnPrivateRepliesToggle);
            let maybe_replies = {
                if item.user_teacher.is_some() || item.user_staff.is_some() && ctx.props().replying_to.is_none() {
                    Some(html! {
                        <label class="checkbox d-flex align-items-center">
                            <input type="checkbox" checked={self.replies_private} onclick={&on_replies_private_toggle} />
                            <span class="ps-3">{lang::dict("Private replies?")}</span>
                        </label>
                    })
                } else {
                    None
                }
            }.unwrap_or(html! {});

            let on_data = ctx
                .link()
                .callback(move |data| MessageCardMessage::OnContent(data));

            let on_send = ctx.link().callback(move |_| MessageCardMessage::Send);

            let on_replies_private_toggle = ctx
                .link()
                .callback(|_: MouseEvent| MessageCardMessage::OnPrivateRepliesToggle);

            let maybe_replies_private = if self.content.len() > 0 {
                maybe_replies
            } else {
                html! {}
            };
            let maybe_replies_student = {
                if item.user_student.is_some() && ctx.props().replying_to.is_some() {
                    Some(html! {
                        <label class="checkbox d-flex align-items-center">
                            <input type="checkbox" checked={self.replies_private} onclick={&on_replies_private_toggle} />
                            <span class="ps-3">{lang::dict("Private replies?")}</span>
                        </label>
                    })
                } else {
                    None
                }
            }.unwrap_or(html! {});
            
            let maybe_replies_private_student = if self.content.len() > 0 {
                maybe_replies_student
            } else {
                html! {}
            };
            let maybe_add = if self.content.len() > 1 {
                html! {
                    <button class="send-message-to-space" onclick={&on_send}>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
                    </button>
                }
            } else {
                html! {
                    <button class="send-message-to-space opacity-75" disabled=true onclick={&on_send}>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
                    </button>
                }
            };
            let maybe_add_direct = if self.content.len() > 1 {
                html! {
                    <button class="send-message-to-space" onclick={&on_send}>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Place")}</span>
                    </button>
                }
            } else {
                html! {
                    <button class="send-message-to-space opacity-75" disabled=true onclick={&on_send}>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Place")}</span>
                    </button>
                }
            };
            let message_class_create = if ctx.props().replying_to.is_some()
                && ctx
                    .props()
                    .message_profile
                    .as_ref()
                    .and_then(|message_profile| Some(MessageId(message_profile.message_id)))
                    != ctx.props().replying_to
            {
                "box-message-to-post bg-white"
            } else {
                "box-message-to-post-2 bg-white"
            };
            let maybe_created = html! {
                <>
                    <div class={message_class_create}>
                        <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                            content={self.content.clone()}
                            upload_url={upload_url.clone()}
                            on_data={on_data.clone()} />
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
                        <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                            content={self.content.clone()}
                            upload_url={upload_url.clone()}
                            on_data={on_data.clone()} />
                    </div>
                    <div class="d-flex justify-content-between align-items-center pt-4 pb-5">
                        <div>{maybe_replies_private.clone()}</div>
                        <div>{ maybe_add_direct.clone() }</div>
                    </div>
                </>
            };
            let maybe_button_message = {
                match ctx.props().group_category {
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
