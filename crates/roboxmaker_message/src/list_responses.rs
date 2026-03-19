use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::message_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{LessonId, PostId, MessageId, GroupId, UserId, RobotId, SchoolId, MyUserProfile};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageGroupCategoryByUser {
    Posts,
    Robots,
    Lessons,
}

pub struct ResponseMessages {
    graphql_task: Option<GraphQLTask>,
    save_task: Option<RequestTask>,
    content: String,
    replies_private: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ResponseMessagesProperties {
    pub user_id: UserId,
    pub message_id: MessageId,
    pub group_id: GroupId,
    pub post_id: Option<PostId>,
    pub robot_id: Option<RobotId>,
    pub lesson_id: Option<LessonId>,
    pub reply_id: Uuid,
    pub user_profile: Option<MyUserProfile>,
    pub category: MessageGroupCategoryByUser,
    pub replying_to: Option<MessageId>,
    pub response_to: String,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum MessageContentSavedUser {
    Lesson(Option<message_model::message_lesson_group_create::ResponseData>),
    Post(Option<message_model::message_post_group_create::ResponseData>),
    Robot(Option<message_model::message_robot_group_create::ResponseData>),
}

#[derive(Debug)]
pub enum ResponseMessagesMessage {
    OnContent(String),
    ContentSaved(MessageContentSavedUser),
    Send,
}

impl Component for ResponseMessages {
    type Message = ResponseMessagesMessage;
    type Properties = ResponseMessagesProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        ResponseMessages {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            save_task: None,
            content: String::default(),
            replies_private: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ResponseMessagesMessage::OnContent(content) => {
                self.content = content;
            }
            ResponseMessagesMessage::ContentSaved(_) => {
                self.content = String::default();
            }
            ResponseMessagesMessage::Send => match ctx.props().category {
                MessageGroupCategoryByUser::Posts => {
                    let content = self.content.clone();
                    let group_id = ctx.props().group_id;
                    let local = chrono::Local::now().naive_local();

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(post_id) = ctx.props().post_id {
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
                                    ResponseMessagesMessage::ContentSaved(MessageContentSavedUser::Post(response))
                                }
                            );
                            self.save_task = Some(task);
                        }
                    }
                },
                MessageGroupCategoryByUser::Robots => {
                    let content = self.content.clone();
                    let group_id = ctx.props().group_id;
                    let local = chrono::Local::now().naive_local();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(robot_id) = ctx.props().robot_id {
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
                                    ResponseMessagesMessage::ContentSaved(MessageContentSavedUser::Robot(response))
                                }
                            );
                            self.save_task = Some(task);
                        }
                    }
                },
                MessageGroupCategoryByUser::Lessons => {
                    let content = self.content.clone();
                    let group_id = ctx.props().group_id;
                    let local = chrono::Local::now().naive_local();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(lesson_id) = ctx.props().lesson_id {
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
                                    ResponseMessagesMessage::ContentSaved(MessageContentSavedUser::Lesson(response))
                                }
                            );
                            self.save_task = Some(task);
                        }
                    }
                },
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_data = ctx.link().callback(move |data| ResponseMessagesMessage::OnContent(data));
        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
        let on_send = ctx.link().callback(move |_| ResponseMessagesMessage::Send);
        let message_id = ctx.props().message_id;
        let class_btn = if self.content.len() > 1 {
            "send-message-to-space"
        } else {
            "send-message-to-space disabled opacity-75"
        };
        let class_btn_disabled = if self.content.len() > 1 {
            false
        } else {
            true
        };
        html! {
            <>
                <div key={message_id.to_string()}>
                    <div class="d-flex flex-column mx-2">
                        <div style="border: 1px solid var(--primary-color); border-radius: 15px; margin-top: 5px; margin-bottom: 5px;">
                            <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                                content={self.content.clone()}
                                upload_url={upload_url.clone()}
                                on_data={on_data.clone()} />
                        </div>
                        <div class="d-flex justify-content-end">
                            <button class={class_btn} onclick={&on_send} disabled={class_btn_disabled}>
                                <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Answer")}</span>
                            </button>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}