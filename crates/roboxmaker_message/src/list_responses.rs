use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::{message_model, school_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{LessonId, PostId, MessageId, GroupId, UserId, RobotId, AppRoute, SchoolId, MyUserProfile};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageGroupCategoryByUser {
    Posts,
    Robots,
    Lessons,
}

pub struct ResponseMessages {
    link: ComponentLink<Self>,
    props: ResponseMessagesProperties,
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
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
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
    AppRoute(AppRoute),
    OnContent(String),
    ContentSaved(MessageContentSavedUser),
    Send,
}

impl Component for ResponseMessages {
    type Message = ResponseMessagesMessage;
    type Properties = ResponseMessagesProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ResponseMessages {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            save_task: None,
            content: String::default(),
            replies_private: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ResponseMessagesMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            ResponseMessagesMessage::OnContent(content) => {
                self.content = content;
            }
            ResponseMessagesMessage::ContentSaved(_) => {
                self.content = String::default();
            }
            ResponseMessagesMessage::Send => match self.props.category {
                MessageGroupCategoryByUser::Posts => {
                    let content = self.content.clone();
                    let group_id = self.props.group_id;
                    let local = chrono::Local::now().naive_local();

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(post_id) = self.props.post_id {
                            let vars = message_model::message_post_group_create::Variables {
                                content,
                                group_id: group_id.0,
                                post_id: post_id.0,
                                reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                                replies_private: self.replies_private,
                                timestamp: local,
                            };
                            let task = message_model::MessagePostGroupCreate::request(
                                graphql_task, 
                                &self.link, 
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
                    let group_id = self.props.group_id;
                    let local = chrono::Local::now().naive_local();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(robot_id) = self.props.robot_id {
                            let vars = message_model::message_robot_group_create::Variables {
                                content,
                                group_id: group_id.0,
                                robot_id: robot_id.0,
                                reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                                replies_private: self.replies_private,
                                timestamp: local,
                            };
                            let task = message_model::MessageRobotGroupCreate::request(
                                graphql_task, 
                                &self.link, 
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
                    let group_id = self.props.group_id;
                    let local = chrono::Local::now().naive_local();
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        if let Some(lesson_id) = self.props.lesson_id {
                            let vars = message_model::message_lesson_group_create::Variables {
                                content,
                                group_id: group_id.0,
                                lesson_id: lesson_id.0,
                                reply_id: self.props.replying_to.and_then(|reply_id| Some(reply_id.0)),
                                replies_private: self.replies_private,
                                timestamp: local,
                            };
                            let task = message_model::MessageLessonGroupCreate::request(
                                graphql_task, 
                                &self.link, 
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
        
        if self.props != props {
            self.props = props;
            should_render = true;
        }
        should_render
    }

    fn view(&self) -> Html {
        let on_data = self.link.callback(move |data| ResponseMessagesMessage::OnContent(data));
        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
        let on_send = self.link.callback(move |_| ResponseMessagesMessage::Send);
        let message_id = self.props.message_id;
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
                <div key=message_id.to_string()>
                    <div class="d-flex flex-column mx-2">
                        <div style="border: 1px solid var(--primary-color); border-radius: 15px; margin-top: 5px; margin-bottom: 5px;">
                            <ckeditor::CKEditor user_profile=self.props.user_profile.clone()
                                content=self.content.clone()
                                upload_url=upload_url.clone()
                                on_data=on_data.clone() />
                        </div>
                        <div class="d-flex justify-content-end">
                            <button class=class_btn onclick=&on_send disabled=class_btn_disabled>
                                <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Answer")}</span>
                            </button>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}