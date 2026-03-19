use log::*;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, post_model::{self, post_by_group_id}};
use roboxmaker_loaders::placeholders::card_post_placeholder::CardPostPlaceholder;
use roboxmaker_types::types::{AppRoute, GroupId, MyUserProfile, PageMode, PostId, SchoolId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};


#[derive(Debug, Clone)]
enum LoadPostFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadPosts {
    Loading,
    Load(LoadPostFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostProfile {
    pub topic: String,
    pub timestamp: String,
    pub maybe_timestamp: String,
    pub post_id: PostId,
    pub full_name: String,
    pub pic_path: String,
}

pub struct PostListHome {
    link: ComponentLink<Self>,
    props: PostListHomeProps,
    graphql_task: Option<GraphQLTask>,
    post_list_task: Option<SubscriptionTask>,
    post_list: Vec<PostProfile>,
    list_posts_state: LoadPosts,

}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostListHomeProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum PostListHomeMessage {
    AppRoute(AppRoute),
    FetchPostsByGroupId,
    Posts(Option<post_model::post_by_group_id::ResponseData>),
}

impl Component for PostListHome {
    type Message = PostListHomeMessage;
    type Properties = PostListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(PostListHomeMessage::FetchPostsByGroupId);

        PostListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            post_list_task: None,
            post_list: vec![],
            list_posts_state: LoadPosts::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            PostListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            PostListHomeMessage::FetchPostsByGroupId => {
                let user = self.props.user_profile.clone();

                self.list_posts_state = LoadPosts::Loading;

                if let Some(user) = user {
                    if user.user_staff.is_some() || user.user_teacher.is_some() {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = self.props.group_id;
        
                            let vars = post_model::post_by_group_id::Variables {
                                group_id: group_id.0,
                                limit: 10,
                                timestamp: Some(post_by_group_id::OrderBy::Desc),
                                maybe_timestamp: None,
                            };
        
                            let task = post_model::PostByGroupId::subscribe(
                                    graphql_task,
                                    &self.link,
                                    vars,
                                    |response| {
                                        PostListHomeMessage::Posts(response)
                                    },
                            );
                            self.post_list_task = Some(task);
                        }
                    } else {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = self.props.group_id;
        
                            let vars = post_model::post_by_group_id::Variables {
                                group_id: group_id.0,
                                limit: 10,
                                maybe_timestamp: Some(post_by_group_id::OrderBy::Desc),
                                timestamp: None,
                            };
        
                            let task = post_model::PostByGroupId::subscribe(
                                    graphql_task,
                                    &self.link,
                                    vars,
                                    |response| {
                                        PostListHomeMessage::Posts(response)
                                    },
                            );
                            self.post_list_task = Some(task);
                        }
                    }
                }
            }
            PostListHomeMessage::Posts(response) => {
                self.post_list = response
                    .clone()
                    .and_then(|data| Some(data.post_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|data| {

                        let topic = data.post_profile.clone().and_then(|d|Some(d.topic)).unwrap_or("".to_string());
                        let post_id = data.post_profile.clone().and_then(|data| Some(data.post_id)).unwrap_or(Uuid::default());

                        let naive = chrono::NaiveDate::from_ymd_opt(2023, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();

                        let timestamp = data.post_profile.clone().and_then(|d| Some(d.timestamp)).unwrap_or(naive);
                        let maybe_timestamp = data.maybe_timestamp.unwrap_or(naive);

                        let time_fn = get_creation_date(timestamp);
                        let maybe_time_fn = get_creation_date(maybe_timestamp);

                        let full_name = data.post_profile.clone().and_then(|d| d.author_profile).and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                        let pic_path = data.post_profile.clone().and_then(|d| d.author_profile).and_then(|data| data.pic_path).unwrap_or("".to_string());
                        
                        PostProfile {
                            topic: topic,
                            timestamp: time_fn,
                            maybe_timestamp: maybe_time_fn,
                            post_id: PostId(post_id),
                            full_name,
                            pic_path,
                        }

                    }).collect();
                if !response.clone().and_then(|data| Some(data.post_group)).unwrap_or(vec![]).is_empty() {
                    self.list_posts_state = LoadPosts::Load(LoadPostFound::Found);
                } else {
                    self.list_posts_state = LoadPosts::Load(LoadPostFound::NotFound);
                }
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(PostListHomeMessage::FetchPostsByGroupId);
        }
        
        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let school_id = self.props.school_id;
        let user = self.props.user_profile.clone();

        let post_card_list = self
            .post_list
            .iter()
            .map(|item| {
            let post_id = item.post_id;
            let on_post_view = self.link.callback(move |_| PostListHomeMessage::AppRoute(AppRoute::Post(school_id, group_id, post_id, PageMode::View)));

            // let maybe_time = if user.user_staff.is_some() || user.user_teacher.is_some() {
            //     html! {
            //         <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
            //     }
            // } else {
            //     html! {
            //         <span class="text-brown noir-light is-size-13 lh-22 ">{&item.maybe_timestamp}</span>
            //     }
            // };
            let maybe_time = if user.clone().and_then(|d| d.user_staff).is_some() || user.clone().and_then(|d| d.user_teacher).is_some() {
                html! {
                    <span class="text-brown noir-light is-size-13 lh-22 ">{ &item.timestamp }</span>
                }
            } else {
                html! {
                    <span class="text-brown noir-light is-size-13 lh-22 ">{ &item.maybe_timestamp }</span>
                }
            };

            html! {
                <div class="card-post-view-home bg-white d-flex flex-column justify-content-between align-items-center p-5 me-5">
                    <a onclick={ &on_post_view }>
                        <div class="module-message-post line-clamp-message-post">
                            <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                { &item.topic.clone() }
                            </span>
                        </div>
                    </a>
                    <a class="w-100" onclick={ &on_post_view }>
                        <div class="d-flex align-items-center justify-content-between">
                            // <img src={ item.pic_path.clone() } class="img-card-32" />
                            // <span class="text-dark noir-light is-size-14 lh-17 text-truncate col-5 mb-0">
                            //     { &item.full_name }
                            // </span>
                            <img src={ item.pic_path.clone() } class="img-card-32" />
                            <span class="text-dark noir-light is-size-14 lh-17 text-truncate col-5 mb-0">
                                { &item.full_name }
                            </span>
                            <div class="ms-2">
                                <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                                    <i class="far fa-clock me-1"></i>
                                    <div class="d-flex flex-wrap">
                                        // <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
                                        { maybe_time }
                                    </div>
                                </span>
                            </div>
                        </div>
                    </a>
                </div>
            }
        }).collect::<Html>();
        let posts_list = match self.list_posts_state {
            LoadPosts::Loading => {
                html! {
                    <>
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                    </>
                }
            },
            LoadPosts::Load(LoadPostFound::Found) => {
                html! {
                    {post_card_list}
                }
            },
            LoadPosts::Load(LoadPostFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No posts here.")}</p>
                    </div>
                }
            },
        };
        html! {
            <div class="d-flex flex-row">   
                { posts_list }
            </div>
        }
    }
}