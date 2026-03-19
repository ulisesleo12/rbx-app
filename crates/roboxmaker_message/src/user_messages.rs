use log::*;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use crate::reply_message::MessageReply;
use serde_derive::{Deserialize, Serialize};
use crate::response_message::MessageResponse;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{list_responses::MessageGroupCategoryByUser, list_responses::ResponseMessages};

use roboxmaker_models::{message_model, school_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_loaders::placeholders::card_comments::CardCommentsPlaceholder;
use roboxmaker_types::types::{AppRoute, GroupId, LessonId, MessageId, MyUserProfile, PageMode, PostId, RobotId, SchoolId, UserId};

#[derive(Debug, Clone)]
enum LoadMessagesReplyFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadMessagesReply {
    Loading,
    Load(LoadMessagesReplyFound),
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessagesContent {
    pub message_id: Uuid,
    pub content: String,
    pub author_full_name: String,
    pub author_pic_path: String,
    pub author_user_id: Uuid,
    pub timestamp: String,
    pub message_profile_reply_id: Uuid,
    pub author_reply_full_name: String,
    pub author_reply_pic_path: String,
    pub author_reply_user_id: Uuid,
    pub content_reply: String,
    pub reply_id: Uuid,
    pub group_id: Uuid,
    pub by_message_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub post_id: Option<Uuid>,
    pub robot_id: Option<Uuid>,
    pub user_id: Uuid,
    pub post_topic: String,
    pub robot_name: String,
    pub lesson_title: String,
    pub post: bool,
    pub lesson: bool,
    pub robot: bool,
    pub school_id: SchoolId,
    pub show_message: bool,
    pub show_modal_response: bool,
}

pub struct MessagesByUserId {
    link: ComponentLink<Self>,
    props: MessagesByUserIdProperties,
    graphql_task: Option<GraphQLTask>,
    replies_list_task: Option<RequestTask>,
    messages: Vec<MessagesContent>,
    list_messages_reply_state: LoadMessagesReply,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MessagesByUserIdProperties {
    pub user_id: UserId,
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
}


#[derive(Debug)]
pub enum MessagesByUserIdMessage {
    AppRoute(AppRoute),
    FetchMessagesByReplies,
    MyMessagesReplies(Option<message_model::my_messages_with_replies::ResponseData>),
    ShowMessage(usize),
    ShowModalMessage(usize),
}

impl Component for MessagesByUserId {
    type Message = MessagesByUserIdMessage;
    type Properties = MessagesByUserIdProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(MessagesByUserIdMessage::FetchMessagesByReplies);
        MessagesByUserId {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            replies_list_task: None,
            messages: vec![],
            list_messages_reply_state: LoadMessagesReply::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MessagesByUserIdMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            MessagesByUserIdMessage::FetchMessagesByReplies => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = message_model::my_messages_with_replies::Variables {
                        author_id: self.props.user_id.0
                    };
                    let task = message_model::MyMessagesWithReplies::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            MessagesByUserIdMessage::MyMessagesReplies(response)
                        }
                    );
                    self.replies_list_task = Some(task);
                }
            }
            MessagesByUserIdMessage::MyMessagesReplies(response) => {
                self.messages = response
                    .clone()
                    .and_then(|data| Some(data.message))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|data_message| {
                        let message_id = data_message.id;
                        let message_profile = data_message.message_profile.clone();
                        let author_full_name = message_profile.clone().and_then(|data| data.author.clone().and_then(|data| Some(data.full_name))).unwrap_or("".to_string());
                        let author_user_id = message_profile.clone().and_then(|data| data.author.clone().and_then(|data| Some(data.user_id))).unwrap_or(Uuid::default());
                        let author_pic_path = message_profile.clone().and_then(|data| data.author.clone().and_then(|data| data.pic_path)).unwrap_or("".to_string());
                        let message_profile_by_reply_id = message_profile.clone().and_then(|data| data.message_profile_by_reply_id).clone();
                        let message_profile_reply_id = message_profile_by_reply_id.clone().and_then(|data| data.message_profile).clone();
                        let timestamp = message_profile_reply_id.clone().and_then(|data| Some(data.timestamp.format("%a %e %b %Y %T").to_string())).unwrap_or("".to_string());
                        let id = message_profile_by_reply_id.clone().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());
                        let author_reply = message_profile_reply_id.clone().and_then(|data| data.author.clone());
                        let author_reply_full_name = author_reply.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                        let author_reply_pic_path = author_reply.clone().and_then(|data| data.pic_path).unwrap_or("".to_string());
                        let author_reply_user_id = author_reply.clone().and_then(|data| Some(data.user_id)).unwrap_or(Uuid::default());
                        let reply_id = message_profile.clone().and_then(|data| data.reply_id).unwrap_or(Uuid::default());
                        let message_group = data_message.message_group.clone();
                        let group_id = message_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
                        let by_message_id = message_group.clone().and_then(|data| Some(data.message_id)).unwrap_or(Uuid::default());
                        let lesson_id = message_group.clone().and_then(|data| data.lesson_id);
                        let post_id = message_group.clone().and_then(|data| data.post_id);
                        let robot_id = message_group.clone().and_then(|data| data.robot_id);
                        let user_id = message_group.clone().and_then(|data| data.user_id).unwrap_or(Uuid::default());
                        let post_topic = message_group.clone().and_then(|data| data.post.clone().and_then(|data| data.post_profile.clone().and_then(|data| Some(data.topic)))).unwrap_or("".to_string());
                        let robot_name = message_group.clone().and_then(|data| data.robot.clone().and_then(|data| data.robot_profile.clone().and_then(|data| Some(data.name)))).unwrap_or("".to_string());
                        let lesson_title = message_group.clone().and_then(|data| data.lesson.clone().and_then(|data| data.lesson_profile.clone().and_then(|data| Some(data.title)))).unwrap_or("".to_string());
                        let post = message_group.clone().and_then(|data| data.post_id);
                        let lesson = message_group.clone().and_then(|data| data.lesson_id);
                        let robot = message_group.clone().and_then(|data| data.robot_id);
                        let school_id = message_group.clone().and_then(|data| data.school_group).clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        MessagesContent {
                            message_id: message_id,
                            content: data_message.message_profile.clone().and_then(|data| data.message_content).clone().and_then(|message_content| Some(message_content.content)).unwrap_or("".to_string()),
                            author_full_name: author_full_name,
                            author_pic_path: author_pic_path,
                            timestamp: timestamp,
                            message_profile_reply_id: id,
                            author_reply_full_name: author_reply_full_name,
                            author_reply_pic_path: author_reply_pic_path,
                            reply_id: reply_id,
                            group_id: group_id,
                            content_reply: data_message.message_profile.clone().and_then(|data| data.message_profile_by_reply_id).clone().and_then(|reply| reply.message_profile).clone().and_then(|message_profile| message_profile.message_content).clone().and_then(|message_content| Some(message_content.content)).unwrap_or("".to_string()),
                            by_message_id: by_message_id,
                            lesson_id: lesson_id,
                            post_id: post_id,
                            robot_id: robot_id,
                            user_id: user_id,
                            post_topic: post_topic,
                            robot_name: robot_name,
                            lesson_title: lesson_title,
                            post: post.is_some(),
                            lesson: lesson.is_some(),
                            robot: robot.is_some(),
                            school_id: SchoolId(school_id),
                            show_message: false,
                            show_modal_response: false,
                            author_user_id: author_user_id,
                            author_reply_user_id: author_reply_user_id,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.message)).unwrap_or(vec![]).is_empty() {
                    self.list_messages_reply_state = LoadMessagesReply::Load(LoadMessagesReplyFound::Found);
                } else {
                    self.list_messages_reply_state = LoadMessagesReply::Load(LoadMessagesReplyFound::NotFound);
                }
            }
            MessagesByUserIdMessage::ShowMessage(idx) => {
                self.messages[idx].show_message = !self.messages[idx].show_message;
            }
            MessagesByUserIdMessage::ShowModalMessage(idx) => {
                self.messages[idx].show_modal_response = !self.messages[idx].show_modal_response;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.user_id != props.user_id {
            self.link.send_message(MessagesByUserIdMessage::FetchMessagesByReplies);
        }
        if self.props != props {
            self.props = props;
            should_render = true;
        }
        
        should_render
    }

    fn view(&self) -> Html {
        let user_id = self.props.user_id;
        let all_messages = self.messages.iter()
            .enumerate().map(|(idx, item)| {
            let on_show_message = self.link.callback(move |_| MessagesByUserIdMessage::ShowMessage(idx));
            let on_show_modal_message = self.link.callback(move |_| MessagesByUserIdMessage::ShowModalMessage(idx));
            let group_id = item.group_id;
            let post_id = item.post_id;
            let robot_id = item.robot_id;
            let lesson_id = item.lesson_id;
            let reply_id = item.reply_id;
            let message_profile_reply_id = item.message_profile_reply_id;
            let post_topic = item.post_topic.clone();
            let robot_name = item.robot_name.clone();
            let lesson_title = item.lesson_title.clone();
            let message_original = html! {
                <>
                    <div class="d-flex align-items-start">
                        <img class="img-card-48" src=item.author_reply_pic_path.clone() />
                    </div>
                    <div class="d-flex flex-column ps-2">
                        <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{&item.author_reply_full_name}</span>
                        <MessageReply content_reply=item.content_reply.clone() 
                            user_message_profile=item.clone() />
                    </div>
                </>
            };
            let maybe_message_original = if item.show_message {
                "card-message-response d-flex align-items-start flex-row p-4 mb-4"
            } else {
                "d-none"
            };
            let post_response = if item.post {
                if let Some(post_id) = post_id {
                    html! {
                        <>
                            <ResponseMessages
                                user_id=self.props.user_id.clone()
                                message_id=MessageId(item.message_id)
                                group_id=GroupId(group_id)
                                post_id=Some(PostId(post_id))
                                robot_id=Some(RobotId(Uuid::default()))
                                lesson_id=Some(LessonId(Uuid::default()))
                                reply_id=reply_id
                                on_app_route=self.props.on_app_route.clone()
                                user_profile=self.props.user_profile.clone()
                                auth_school=self.props.auth_school.clone()
                                replying_to=MessageId(message_profile_reply_id)
                                response_to=post_topic
                                school_id=item.school_id
                                category=MessageGroupCategoryByUser::Posts />
                        </>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            let lessons_response = if item.lesson {
                if let Some(lesson_id) = lesson_id {
                    html! {
                        <>
                            <ResponseMessages
                                user_id=self.props.user_id.clone()
                                message_id=MessageId(item.message_id)
                                group_id=GroupId(group_id)
                                post_id=Some(PostId(Uuid::default()))
                                robot_id=Some(RobotId(Uuid::default()))
                                lesson_id=Some(LessonId(lesson_id))
                                reply_id=reply_id
                                on_app_route=self.props.on_app_route.clone()
                                user_profile=self.props.user_profile.clone()
                                auth_school=self.props.auth_school.clone()
                                replying_to=MessageId(message_profile_reply_id)
                                response_to=lesson_title
                                school_id=item.school_id
                                category=MessageGroupCategoryByUser::Lessons />
                        </>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            let robots_response = if item.robot {
                if let Some(robot_id) = robot_id {
                    html! {
                        <>
                            <ResponseMessages
                                user_id=self.props.user_id.clone()
                                message_id=MessageId(item.message_id)
                                group_id=GroupId(group_id)
                                post_id=Some(PostId(Uuid::default()))
                                robot_id=Some(RobotId(robot_id))
                                lesson_id=Some(LessonId(Uuid::default()))
                                reply_id=reply_id
                                on_app_route=self.props.on_app_route.clone()
                                user_profile=self.props.user_profile.clone()
                                auth_school=self.props.auth_school.clone()
                                replying_to=MessageId(message_profile_reply_id)
                                response_to=robot_name
                                school_id=item.school_id
                                category=MessageGroupCategoryByUser::Robots />
                        </>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            let class_modal_response = if item.show_modal_response {
                "modal fade show"
            } else {
                "modal fade"
            };
            let class_reponse_scroll = if item.show_modal_response {
                "display: block;"
            } else {
                "display: none;"
            };
            let group_id = GroupId(item.group_id);
            let school_id = item.school_id;
            let maybe_post_to_go = if item.post {
                if let Some(post_id) = post_id {
                    let post_id = PostId(post_id);
                    let on_post = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::Post(school_id, group_id , post_id, PageMode::View)));
                    html! {
                        <a class="text-secondary-purple noir-medium is-size-14 lh-17 col-2 text-truncate text-end" onclick=on_post>{lang::dict("Come in for ")}{&item.post_topic.clone()}</a>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            let maybe_post_to_go_two = if item.post {
                if let Some(post_id) = post_id {
                    let post_id = PostId(post_id);
                    let on_post = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::Post(school_id, group_id , post_id, PageMode::View)));
                    html! {
                        <div class="d-flex align-items-center justify-content-end bg-white-bis px-2 py-3">
                            <a onclick=on_post>
                                <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{lang::dict("Come in for ")}{&item.post_topic.clone()}</span>
                            </a>
                        </div>
                        // <a class="text-secondary-purple noir-medium is-size-14 lh-17 col-2 text-truncate text-end" onclick=on_post>{lang::dict("Come in for ")}{&item.post_topic.clone()}</a>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            let maybe_lesson_to_go = if item.lesson {
                if let Some(lesson_id) = lesson_id {
                    let lesson_id = LessonId(lesson_id);
                    let on_lesson = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::LessonView(school_id, group_id, lesson_id)));
                    html! {
                        <a class="text-secondary-purple noir-medium is-size-14 lh-17 col-2 text-truncate text-end" onclick=on_lesson>{lang::dict("Come in for ")}{&item.lesson_title.clone()}</a>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };                                      
            let maybe_lesson_to_go_two = if item.lesson {
                if let Some(lesson_id) = lesson_id {
                    let lesson_id = LessonId(lesson_id);
                    let on_lesson = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::LessonView(school_id, group_id, lesson_id)));
                    html! {
                        <div class="d-flex align-items-center justify-content-end bg-white-bis px-2 py-3">
                            <a onclick=on_lesson>
                                <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{lang::dict("Come in for ")}{&item.lesson_title.clone()}</span>
                            </a>
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };                                      
            let maybe_robot_to_go = if item.robot {
                if let Some(robot_id) = robot_id {
                    let user_id = UserId(Uuid::default());
                    let robot_id = RobotId(robot_id);
                    let on_robot = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::Robot(robot_id, group_id, user_id)));
                    html! {
                        <a class="text-secondary-purple noir-medium is-size-14 lh-17 col-2 text-truncate text-end" onclick=on_robot>{lang::dict("Come in for ")}{&item.robot_name.clone()}</a>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };  
            let maybe_robot_to_go_two = if item.robot {
                if let Some(robot_id) = robot_id {
                    let user_id = UserId(Uuid::default());
                    let robot_id = RobotId(robot_id);
                    let on_robot = self.link.callback(move |_| MessagesByUserIdMessage::AppRoute(AppRoute::Robot(robot_id, group_id, user_id)));
                    html! {
                        <div class="d-flex align-items-center justify-content-end bg-white-bis px-2 py-3">
                            <a onclick=on_robot>
                                <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{lang::dict("Come in for ")}{&item.robot_name.clone()}</span>
                            </a>
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };  
            let maybe_original_message = if item.author_user_id != user_id.0 {
                html! {
                    <div class="card-comments-user d-flex align-items-center mb-4 w-100">
                        <div class="p-4 d-flex flex-row w-100">
                            <img class="img-card-48" src=item.author_pic_path.clone() />
                            <div class="d-flex flex-column w-100 ps-2">
                                <div class="d-flex align-items-center justify-content-between">
                                    <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{&item.author_full_name}{" te ha respondido"}</span>
                                    {maybe_post_to_go}
                                    {maybe_lesson_to_go}
                                    {maybe_robot_to_go}
                                </div>
                                <span class="text-brown noir-light is-size-16 lh-19">
                                    <MessageResponse content=item.content.clone()
                                        user_message_profile=item.clone() />
                                </span>
                                <div class="d-flex align-items-center flex-row">
                                    <span class="text-secondary-purple noir-medium is-size-14 lh-17 pe-5 me-2">{&item.timestamp}</span>
                                    <a onclick=&on_show_message class="btn btn-white text-secondary-purple noir-medium is-size-14 lh-17 pe-5 me-2">{"Ver Comentario"}</a>
                                    <a class="btn btn-white text-primary-blue-dark noir-medium is-size-14 lh-17" onclick=&on_show_modal_message>{"Responder"}</a>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            };
            html! {
                <>
                    <div class=maybe_message_original>{message_original}</div>
                    {maybe_original_message}
                    <div class=class_modal_response id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style=class_reponse_scroll aria-modal="true" role="dialog">
                        <div class="modal-dialog modal-dialog-scrollable modal-xl">
                            <div class="modal-content">
                                <div class="modal-header">
                                    <h1 class="text-primary-blue-dark noir-bold is-size-36 lh-43 text-uppercase m-0">{lang::dict("Answer")}</h1>
                                    <a class="btn bg-purple-on ms-5" onclick=&on_show_modal_message>
                                        <span class="text-white">
                                            <i class="fas fa-times"></i>
                                        </span>
                                    </a>
                                </div>
                                <div class="modal-body px-5 vh-100">
                                    <div class="d-flex flex-row">
                                        <div class="d-flex align-items-start">
                                            <img class="img-card-48" src=item.author_reply_pic_path.clone() />
                                        </div>
                                        <div class="d-flex flex-column ps-2">
                                            <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{&item.author_reply_full_name}</span>
                                            <span class="text-brown noir-light is-size-16 lh-19">
                                                <MessageReply content_reply=item.content_reply.clone()
                                                    user_message_profile=item.clone() />
                                            </span>
                                        </div>
                                    </div>
                                    <div class="d-flex align-items-center m-4 ms-5 p-4 border rounded bg-lavanda-light">
                                        <div class="box d-flex flex-row w-100">
                                            <img class="img-card-48" src=item.author_pic_path.clone() />
                                            <div class="d-flex flex-column ps-2">
                                                <span class="text-primary-blue-dark noir-bold is-size-16 lh-19">{&item.author_full_name}{" te ha
                                                    respondido"}</span>
                                                    <div class="text-brown noir-light is-size-16 lh-19">
                                                        <MessageResponse content=item.content.clone()
                                                            user_message_profile=item.clone() />
                                                    </div>
                                                <div class="d-flex flex-row">
                                                    <span class="text-secondary-purple noir-medium is-size-14 lh-17 pe-5 me-2">{&item.timestamp}</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    {maybe_post_to_go_two}
                                    {maybe_lesson_to_go_two}
                                    {maybe_robot_to_go_two}
                                    {post_response}
                                    {lessons_response}
                                    {robots_response}
                                </div>
                            </div>
                        </div>
                    </div>
                </>
            }
        }).collect::<Html>();
        let messages_list = match self.list_messages_reply_state {
            LoadMessagesReply::Loading => {
                html! {
                    <CardCommentsPlaceholder />
                }
            },
            LoadMessagesReply::Load(LoadMessagesReplyFound::Found) => {
                html! {
                    {all_messages}
                }
            },
            LoadMessagesReply::Load(LoadMessagesReplyFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No posts here.")}</p>
                    </div>
                }
            },
        };
        html! {
            {messages_list}
        }
    }
}