use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use serde_derive::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::user_model;
use roboxmaker_types::types::UserId;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_message::{reply_message::MessageReply, response_message::MessageResponse};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessagesContent {
    pub message_id: Uuid,
    pub content: String,
    pub author_full_name: String,
    pub timestamp: String,
    pub author_reply_full_name: String,
    pub message_profile_reply_id: Uuid,
    pub content_reply: String,
    pub post_topic: String,
    pub robot_name: String,
    pub lesson_title: String,
    pub post: bool,
    pub lesson: bool,
    pub robot: bool,
    pub show_message: bool,
}

pub struct ContributionsAndComments {
    link: ComponentLink<Self>,
    props: ContributionsAndCommentsProperties,
    graphql_task: Option<GraphQLTask>,
    task_my_comments: Option<RequestTask>,
    messages: Vec<MessagesContent>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ContributionsAndCommentsProperties {
    pub user_id: UserId,
}

#[derive(Debug)]
pub enum ContributionsAndCommentsMessage {
    FetchMyContributionsAndComments,
    MyContributionsAndComments(Option<user_model::my_contributions_and_comments::ResponseData>),
    ShowMessage(usize)
}

impl Component for ContributionsAndComments {
    type Message = ContributionsAndCommentsMessage;
    type Properties = ContributionsAndCommentsProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ContributionsAndCommentsMessage::FetchMyContributionsAndComments);
        ContributionsAndComments {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task_my_comments: None,
            messages: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ContributionsAndCommentsMessage::FetchMyContributionsAndComments => {
                let author_id = self.props.user_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = user_model::my_contributions_and_comments::Variables { 
                        author_id: author_id.0
                    };

                    let task = user_model::MyContributionsAndComments::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            ContributionsAndCommentsMessage::MyContributionsAndComments(response)
                        },
                    );
                    self.task_my_comments = Some(task);
                }
            }
            ContributionsAndCommentsMessage::MyContributionsAndComments(response) => {
                self.messages = response
                    .clone()
                    .and_then(|data| Some(data.message))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|data_message| {
                        let message_id = data_message.id;
                        let message_profile = data_message.message_profile.clone();
                        let author_full_name = message_profile.clone().and_then(|data| data.author.clone().and_then(|data| Some(data.full_name))).unwrap_or("".to_string());
                        let message_profile_by_reply_id = message_profile.clone().and_then(|data| data.message_profile_by_reply_id).clone();
                        let message_profile_reply_id = message_profile_by_reply_id.clone().and_then(|data| data.message_profile).clone();
                        let id = message_profile_by_reply_id.clone().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());
                        let timestamp = message_profile_reply_id.clone().and_then(|data| Some(data.timestamp.format("%a %e %b %Y").to_string())).unwrap_or("".to_string());
                        
                        let author_reply = message_profile_reply_id.clone().and_then(|data| data.author.clone());
                        let author_reply_full_name = author_reply.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                        let message_content = message_profile_reply_id.clone().and_then(|data| data.message_content);
                        let message_group = message_content.clone().and_then(|data| data.message_group);
                        let post_topic = message_group.clone().and_then(|data| data.post.clone().and_then(|data| data.post_profile.clone().and_then(|data| Some(data.topic)))).unwrap_or("".to_string());
                        let robot_name = message_group.clone().and_then(|data| data.robot.clone().and_then(|data| data.robot_profile.clone().and_then(|data| Some(data.name)))).unwrap_or("".to_string());
                        let lesson_title = message_group.clone().and_then(|data| data.lesson.clone().and_then(|data| data.lesson_profile.clone().and_then(|data| Some(data.title)))).unwrap_or("".to_string());
                        let post = message_group.clone().and_then(|data| data.post.clone().and_then(|data| Some(data.id)));
                        let lesson = message_group.clone().and_then(|data| data.lesson.clone().and_then(|data| Some(data.id)));
                        let robot = message_group.clone().and_then(|data| data.robot.clone().and_then(|data| Some(data.id)));
                        MessagesContent {
                            message_id: message_id,
                            author_full_name: author_full_name,
                            timestamp: timestamp,
                            message_profile_reply_id: id,
                            author_reply_full_name: author_reply_full_name,
                            post_topic: post_topic,
                            robot_name: robot_name,
                            lesson_title: lesson_title,
                            post: post.is_some(),
                            lesson: lesson.is_some(),
                            robot: robot.is_some(),
                            show_message: false,
                            content: data_message.message_profile.clone().and_then(|data| data.message_content).clone().clone().and_then(|message_content| Some(message_content.content)).unwrap_or("".to_string()),
                            content_reply: data_message.message_profile.clone().and_then(|data| data.message_profile_by_reply_id).clone().clone().and_then(|message_reply| message_reply.message_profile).clone().and_then(|mesage_profile| mesage_profile.message_content).clone().and_then(|message_content| Some(message_content.content)).unwrap_or("".to_string()),
                        }
                    }).collect();
            }
            ContributionsAndCommentsMessage::ShowMessage(idx) => {
                self.messages[idx].show_message = !self.messages[idx].show_message
            }
        }
        should_update
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

        let all_messages = self.messages.iter().enumerate().map(|(idx, item)| {
            // let message_profile_reply_id = item.message_profile_reply_id;
            // let message_id = item.message_id;
            let on_show_message = self.link.callback(move |_| ContributionsAndCommentsMessage::ShowMessage(idx));
            let maybe_view_content_reply = if item.show_message {
                "module-message-universal-3 line-clamp-message-universal-4 pb-2"
            } else {
                "module-message-universal-3 line-clamp-message-universal-3"
            };
            let maybe_view_content = if item.show_message {
                "module-message-universal-3 line-clamp-message-universal-4 pb-2"
            } else {
                "module-message-universal-3 line-clamp-message-universal-3"
            };
            let maybe_view_mensaje = if item.show_message {
                "py-3"
            } else {
                "d-none"
            };
            let card_message_view = if item.show_message {
                "container-maybe-message-2 mb-4"
            } else {
                "container-maybe-message mb-4"
            };
            let posts_topic = if item.post {
                html! {
                    <>
                        <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&item.post_topic}</span>
                    </>
                }
            } else {
                html! {}
            };
            let lessons_title = if item.lesson {
                html! {
                    <>
                        <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&item.lesson_title}</span>
                    </>
                }
            } else {
                html! {}
            };
            let robots_name = if item.robot {
                html! {
                    <>
                        <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&item.robot_name}</span>
                    </>
                }
            } else {
                html! {}
            };
            html! {
                <>
                    <div class=card_message_view>
                        <div class="d-flex flex-column justify-content-between p-4">
                            <div
                                class="d-flex flex-wrap align-items-center justify-content-between">
                                {posts_topic}
                                {lessons_title}
                                {robots_name}
                                <span class="icon" style="color: #827A89;">
                                    <i class="fas fa-ellipsis-v"></i>
                                </span>
                            </div>
                            <div class=maybe_view_mensaje>
                                <div class=maybe_view_content>
                                    <span class="text-gray-purple noir-regular is-size-14 lh-18-2">
                                        <MessageResponse content=item.content.clone() />
                                    </span>
                                </div>
                            </div>
                            <div class=maybe_view_content_reply>
                                <span class="text-gray-purple noir-regular is-size-14 lh-18">
                                    <MessageReply content_reply=item.content_reply.clone() />
                                </span>
                            </div>
                            <div class="d-flex flex-wrap justify-content-between">
                                <span class="text-secondary-purple noir-bold is-size-14 lh-18">{&item.timestamp}</span>
                                <a onclick=on_show_message class="text-secondary-purple noir-bold is-size-14 lh-18">{"Ver Comentario"}</a>
                            </div>
                        </div>
                    </div>
                </>
            }
        }).collect::<Html>();
        html! {
            <div class="container-messages-user-profile">
                {all_messages}
            </div>
        }
    }
}