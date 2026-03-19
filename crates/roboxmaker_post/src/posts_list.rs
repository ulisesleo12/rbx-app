use log::*;
use uuid::Uuid;
use yew::prelude::*;
use chrono::Local;
use code_location::code_location;
use crate::post_select::PostSelect;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{posts_card::PostCard, post_select::PostSelectOption};


use roboxmaker_main::lang;
use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_searches::search_posts_group::SearchPostsGroup;
use roboxmaker_loaders::placeholders::card_post_list::CardPostListPlaceholder;
use roboxmaker_models::{post_model::{self, posts_list_by_group}, school_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, AppRoute, PostId, LoadResponseFound, LoadResponse, SchoolId, MyUserProfile, PageMode};

#[derive(Debug, Clone, PartialEq)]
pub struct PostProfile {
    pub topic: String,
    pub timestamp: String,
    pub timestamp_published: String,
    pub post_id: PostId,
    pub author_id: Uuid,
    pub author_pic_path: String,
    pub author_full_name: String,
    pub shares: i64,
    pub archived: bool,
    pub published: bool,
    pub on_dropdown_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PostFilter {
    Alls,
    Published,
    Unpublished,
    Archived,
}

pub struct PostsList {
    link: ComponentLink<Self>,
    props: PostsListProps,
    graphql_task: Option<GraphQLTask>,
    post_sub: Option<SubscriptionTask>,
    post_delete_task: Option<RequestTask>,
    post_add_task: Option<RequestTask>,
    post_list: Vec<PostProfile>,
    filter: PostFilter,
    show_dropdown_filter: bool,
    list_response_state: LoadResponse,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostsListProps {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub class_name: String,
    pub inventory_group: Option<Uuid>,
}

#[derive(Debug)]
pub enum PostsListMessage {
    AppRoute(AppRoute),
    FetchPostsByGroupId,
    Posts(Option<post_model::posts_list_by_group::ResponseData>),
    ChangeFilter(PostFilter),
    ShowDropdownFilter,
    RemovePost(PostId),
    RemovePostEntirely(PostId),
    DeletePost(PostId),
    PostRemoved(Option<PostId>),
    CreatePost,
    AddPost(PostId),
    PostAdded(Option<PostId>),
    UpdatePostList(PostId, bool, bool),
    // DeleteQuiz(Uuid),
    // DeleteQuizResp(Option<quiz_model::delete_quiz_by_id::ResponseData>),
}

impl Component for PostsList {
    type Message = PostsListMessage;
    type Properties = PostsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(PostsListMessage::FetchPostsByGroupId);

        PostsList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            post_sub: None,
            post_delete_task: None,
            post_add_task: None,
            post_list: vec![],
            filter: PostFilter::Alls,
            show_dropdown_filter: false,
            list_response_state: LoadResponse::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            PostsListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            PostsListMessage::FetchPostsByGroupId => {
                let user = self.props.user_profile.clone();

                if let Some(item) = user {
                    if item.user_staff.is_some() || item.user_teacher.is_some() {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = self.props.group_id;
        
                            let vars = post_model::posts_list_by_group::Variables {
                                group_id: group_id.0,
                                timestamp: Some(posts_list_by_group::OrderBy::Desc),
                                maybe_timestamp: None, 
                            };
        
                            let task = post_model::PostsListByGroup::subscribe(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    PostsListMessage::Posts(response)
                                },
                            );
                            self.post_sub = Some(task);
                        }
                    } else {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = self.props.group_id;
        
                            let vars = post_model::posts_list_by_group::Variables {
                                group_id: group_id.0,
                                maybe_timestamp: Some(posts_list_by_group::OrderBy::Desc), 
                                timestamp: None,
                            };
        
                            let task = post_model::PostsListByGroup::subscribe(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    PostsListMessage::Posts(response)
                                },
                            );
                            self.post_sub = Some(task);
                        }
                    }
                }

            }
            PostsListMessage::Posts(response) => {
                self.post_list = response
                    .clone()
                    .and_then(|data| Some(data.post_group))
                    .unwrap_or_default()
                    .iter()
                    .filter(|posts| {

                        let archived = posts.archived.clone();
                        let published = posts.published.clone();

                        self.filter == PostFilter::Alls && archived == false ||
                
                        self.filter == PostFilter::Published && published == true && archived == false ||
        
                        self.filter == PostFilter::Unpublished && archived == false && published == false ||
        
                        self.filter == PostFilter::Archived && archived == true && published == false
                    })
                    .map(|item| {
                        let post_profile = item.post_profile.clone();
                        
                        let topic = post_profile.clone().and_then(|data| Some(data.topic)).unwrap_or("".to_string());
                        let id = post_profile.clone().and_then(|data| Some(data.post_id)).unwrap_or(Uuid::default());
                        let author_id = post_profile.clone().and_then(|data| Some(data.author_id)).unwrap_or(Uuid::default());
                        let author_pic_path = post_profile.clone().and_then(|data| data.author_profile).and_then(|author_profile| author_profile.pic_path).unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_string());
                        let author_full_name = post_profile.clone().and_then(|data| data.author_profile).and_then(|author_profile| Some(author_profile.full_name)).unwrap_or("".to_string());
                        let archived = item.archived.clone();
                        let published = item.published.clone();
                        

                        let naive = chrono::NaiveDate::from_ymd_opt(2023, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();

                        let timestamp = post_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive);
                        
                        let time_fn = get_creation_date(timestamp);
                        
                        let shares = item.message_posts.len();
                        
                        
                        let timestamp_published = item.maybe_timestamp.unwrap_or(naive);
                        let time_published_fn = get_creation_date(timestamp_published);

                        PostProfile {
                            topic: topic.clone(),
                            timestamp: time_fn,
                            timestamp_published: time_published_fn,
                            post_id: PostId(id),
                            author_id,
                            author_pic_path,
                            author_full_name,
                            shares: shares.try_into().unwrap_or(0),
                            archived,
                            published,
                            on_dropdown_menu: false,

                        }
                    }).collect();

                if !response.clone().and_then(|data| Some(data.post_group)).unwrap_or(vec![]).is_empty() {
                    self.list_response_state = LoadResponse::Load(LoadResponseFound::Found);
                } else {
                    self.list_response_state = LoadResponse::Load(LoadResponseFound::NotFound);
                }

            }
            PostsListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                self.link.send_message(PostsListMessage::FetchPostsByGroupId);
            }
            PostsListMessage::ShowDropdownFilter => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            PostsListMessage::CreatePost => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let local = Local::now().naive_local();

                    if let Some(inventory_group_id) = self.props.inventory_group {
                        let vars = post_model::post_group_create::Variables { 
                            topic: String::from(lang::dict("~ New Post ~")),
                            content: String::from(""),
                            group_id: self.props.group_id.0,
                            inventory_group_id,
                            timestamp: local,
                            post_id: Uuid::new_v4(),
                        };
    
                        let task = post_model::PostGroupCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                let post_id = if let Some(post) = response {
                                    post.insert_post_group_one.and_then(|data| Some(PostId(data.post_id)))
                                } else {
                                    None
                                };
                                PostsListMessage::PostAdded(post_id)
                            },
                        );
                        self.post_add_task = Some(task);
                        self.link.send_message(PostsListMessage::FetchPostsByGroupId);
                    }
                }
            }
            PostsListMessage::RemovePost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = post_model::post_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let post_id = if let Some(response) = response {
                                if response.delete_post_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(PostId(response.delete_post_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].post_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            PostsListMessage::PostRemoved(post_id)
                        },
                    );
                    self.post_delete_task = Some(task);
                }
            }
            PostsListMessage::RemovePostEntirely(post_id) => {
                self.link.send_message(PostsListMessage::DeletePost(post_id));
            }
            PostsListMessage::DeletePost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = post_model::delete_post_by_id::Variables { 
                        post_id: post_id.0,
                    };

                    let task = post_model::DeletePostById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        move |response| {
                            let post_id = if let Some(response) = response {
                                if response.delete_post_by_pk.clone().and_then(|data| Some(data.id)).is_some() {
                                    let id = response.delete_post_by_pk.clone().and_then(|data| Some(data.id)).unwrap();
                                    Some(PostId(id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            PostsListMessage::PostRemoved(post_id)
                        },
                    );
                    self.post_delete_task = Some(task);
                }
            }
            PostsListMessage::PostRemoved(post_id) => {
                if post_id.is_some() {
                    info!("DELETE POST: {}", post_id.unwrap().to_string());
                }
                // if let Some(post_id) = post_id {
                //     self.post_list.retain(|u| u.post_id != post_id);
                // } else {
                //     should_update = false;
                // }
            }
            // PostsListMessage::DeleteQuiz(quiz_id) => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = quiz_model::delete_quiz_by_id::Variables { 
            //             quiz_id
            //         };

            //         let task = quiz_model::DeleteQuizById::request(
            //             graphql_task,
            //             &self.link,
            //             vars,
            //             |response| {
            //                 PostsListMessage::DeleteQuizResp(response)
            //             },
            //         );
            //         self.post_delete_task = Some(task);
            //     }
            // }
            // PostsListMessage::DeleteQuizResp(response) => {
            //     if response.is_some() {
            //         info!("DeleteQuizResp: {:?}", response);
            //     }
            // }
            PostsListMessage::AddPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = post_model::post_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let post_id = if let Some(post) = response {
                                post.insert_post_group_one.and_then(|data| Some(PostId(data.post_id)))
                            } else {
                                None
                            };
                            PostsListMessage::PostAdded(post_id)
                        },
                    );
                    self.post_add_task = Some(task);
                }
            }
            PostsListMessage::PostAdded(post_id) => {
                let group_id = self.props.group_id;
                let school_id = self.props.school_id;

                if let Some(post_id) = post_id {
                    self.link.send_message(PostsListMessage::AppRoute(AppRoute::Post(school_id, group_id, post_id, PageMode::Edit)));

                } else {
                    should_update = true;
                }
            }
            PostsListMessage::UpdatePostList(post_id, published , archived) => {
                for post in self.post_list.iter_mut() {
                    if post.post_id == post_id {
                        post.archived = archived;
                        post.published = published;
                    }
                }
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
        let group_id = self.props.group_id;
        let on_alls = self.link.callback(|_| PostsListMessage::ChangeFilter(PostFilter::Alls));
        let on_published = self.link.callback(|_| PostsListMessage::ChangeFilter(PostFilter::Published));
        let on_unpublished = self.link.callback(|_| PostsListMessage::ChangeFilter(PostFilter::Unpublished));
        let on_archived = self.link.callback(|_| PostsListMessage::ChangeFilter(PostFilter::Archived));
        let on_dropdown = self.link.callback(|_| PostsListMessage::ShowDropdownFilter);
        let on_change_list = self.link.callback(|(post_id, published, archived)| PostsListMessage::UpdatePostList(post_id, published, archived));

        let on_post_delete = self.link.callback(|post_id| PostsListMessage::RemovePost(post_id));
        let on_post_delete_entirely = self.link.callback(|post_id| PostsListMessage::RemovePostEntirely(post_id));

        let maybe_option_seleted = match self.filter {
            PostFilter::Alls => "Everyone",
            PostFilter::Published => "Released",
            PostFilter::Unpublished => "Unpublished",
            PostFilter::Archived => "Archived",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };

        let maybe_dropdown_by_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_published>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Published {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_unpublished>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Unpublished {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_archived>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Archived {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Archived {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Archived")}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let posts_list = self.post_list.iter()
            .map(|item| {            

            html! {
                <PostCard 
                    group_id={ self.props.group_id }
                    school_id={ self.props.school_id }
                    on_app_route={ self.props.on_app_route.clone() }
                    user_profile={ self.props.user_profile.clone() }
                    on_post_delete={ {on_post_delete.clone()} }
                    on_change_list={ on_change_list.clone() }
                    topic={ item.topic.clone() }
                    timestamp={ item.timestamp.clone() }
                    timestamp_published={ item.timestamp_published.clone() }
                    shares={ {item.shares} }
                    post_id={ item.post_id }
                    author_id={ item.author_id }
                    author_pic_path={ item.author_pic_path.clone() }
                    author_full_name={ item.author_full_name.clone() }
                    archived={ item.archived }
                    published={ item.published }
                    on_dropdown_menu={ item.on_dropdown_menu }
                    on_post_delete_entirely={ on_post_delete_entirely.clone() } />
            }
        }).collect::<Html>();

        let maybe_option = match self.list_response_state {
            LoadResponse::Loading => {
                html! {
                    <div class="d-flex flex-wrap">
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap">
                        {posts_list}
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No posts here.")}</span>
                    </div>
                }
            },
        };
        let maybe_post_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    PostSelectOption::Post(post_id) => PostsListMessage::AddPost(post_id)
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <PostSelect on_select=on_select 
                            allow_create=true
                            school_id=self.props.school_id
                            group_id=group_id.clone()
                            user_profile=self.props.user_profile.clone()
                            on_app_route=self.props.on_app_route.clone() />
                    })
                } else {
                    Some(html! {
                        <SearchPostsGroup on_app_route=self.props.on_app_route.clone()
                            user_profile=self.props.user_profile.clone()
                            group_id=self.props.group_id
                            school_id=self.props.school_id />
                    })
                }
            })
            .unwrap_or(html! {});
        let maybe_user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
        let on_direct_meet = self.link.callback(move |_| PostsListMessage::AppRoute(AppRoute::MeetDirect(group_id)));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {maybe_post_search}
                {maybe_user_profile_pic}
            </div>
        };

        let maybe_new = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    let on_select = self.link.callback(move |_| PostsListMessage::CreatePost);
                    Some(html! {
                        <>
                            <a class="button btn-create-card d-flex align-items-center justify-content-center" onmousedown=on_select.clone()>
                                <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                                    <i class="fas fa-plus me-2"></i>
                                    <span>{lang::dict("New Post")}</span>
                                </span>
                            </a>
                        </>
                    })
                } else {Some(html! {})}
            })
            .unwrap_or(html! {});

        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Posts")} <span class="ps-1">{"("}{self.post_list.iter().cloned().len()}{")"}</span>
                </span>
                <div class="d-flex flex-wrap">
                    {maybe_dropdown_by_user}
                    {maybe_new}
                </div>
            </div>
        };
        html! {
            <>
                <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7">
                    {head_section}
                    {maybe_dropdown}
                    {maybe_option}
                </div>
            </>
        }
    }
}