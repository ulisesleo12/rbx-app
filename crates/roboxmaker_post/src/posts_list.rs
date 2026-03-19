use log::*;
use uuid::Uuid;
use yew::prelude::*;
use chrono::Local;
use yew::{html, Component, Html};
use code_location::code_location;
use crate::post_select::PostSelect;
use yew_router::scope_ext::RouterScopeExt;
use crate::{posts_card::PostCard, post_select::PostSelectOption};


use roboxmaker_main::lang;
use roboxmaker_utils::functions::get_creation_date;
use roboxmaker_searches::search_posts_group::SearchPostsGroup;
use roboxmaker_models::post_model::{self, get_post_list};
use roboxmaker_loaders::placeholders::card_post_list::CardPostListPlaceholder;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, AppRoute, PostId, LoadResponseFound, LoadResponse, SchoolId, MyUserProfile};

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
    graphql_task: Option<GraphQLTask>,
    post_sub: Option<SubscriptionTask>,
    post_delete_task: Option<RequestTask>,
    post_add_task: Option<RequestTask>,
    post_list: Vec<PostProfile>,
    post_list_view: Vec<PostProfile>,
    filter: PostFilter,
    show_dropdown_filter: bool,
    list_response_state: LoadResponse,
    section_id: Option<Uuid>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostsListProps {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub class_name: String,
    #[prop_or(None)]
    pub inventory_group: Option<Uuid>,
}

#[derive(Debug)]
pub enum PostsListMessage {
    // AppRoute(AppRoute),
    FetchPostsByGroupId,
    Posts(Option<post_model::get_post_list::ResponseData>),
    ChangeFilter(PostFilter),
    ShowDropdownFilter,
    RemovePost(PostId),
    RemovePostEntirely(PostId),
    PostRemoved(Option<PostId>),
    CreatePost,
    AddPost(PostId),
    PostAdded(Option<PostId>),
    UpdatePostList(PostId, bool, bool),
}

impl Component for PostsList {
    type Message = PostsListMessage;
    type Properties = PostsListProps;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(PostsListMessage::FetchPostsByGroupId);

        PostsList {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            post_sub: None,
            post_delete_task: None,
            post_add_task: None,
            post_list: vec![],
            post_list_view: vec![],
            filter: PostFilter::Alls,
            show_dropdown_filter: false,
            list_response_state: LoadResponse::Loading,
            section_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            PostsListMessage::FetchPostsByGroupId => {
                let user = ctx.props().user_profile.clone();

                if let Some(item) = user {
                    if item.user_staff.is_some() || item.user_teacher.is_some() {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = ctx.props().group_id;
        
                            let vars = post_model::get_post_list::Variables {
                                group_id: group_id.0,
                                timestamp: Some(get_post_list::OrderBy::Desc),
                                maybe_timestamp: None, 
                            };
        
                            let task = post_model::GetPostList::subscribe(
                                graphql_task,
                                &ctx,
                                vars,
                                |response| {
                                    PostsListMessage::Posts(response)
                                },
                            );
                            self.post_sub = Some(task);
                        }
                    } else {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let group_id = ctx.props().group_id;
        
                            let vars = post_model::get_post_list::Variables {
                                group_id: group_id.0,
                                maybe_timestamp: Some(get_post_list::OrderBy::Desc), 
                                timestamp: None,
                            };
        
                            let task = post_model::GetPostList::subscribe(
                                graphql_task,
                                &ctx,
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
                if let Some(class_group) = response.clone().and_then(|data| Some(data.class_group)) {

                    for class_post in class_group.iter() {
                        self.section_id = class_post.class_profile.clone().and_then(|data| Some(data.section_id));

                        let posts = class_post.class_profile.clone().and_then(|data| Some(data.class_post)).unwrap_or(vec![]);
                        let post_group = class_post.post_groups.clone();
                        
                        // if !response.clone().and_then(|data| Some(data.post_group)).unwrap_or(vec![]).is_empty() {
                        if !posts.is_empty() {
                            self.list_response_state = LoadResponse::Load(LoadResponseFound::Found);
                        } else {
                            self.list_response_state = LoadResponse::Load(LoadResponseFound::NotFound);
                        }

                        let post_list = posts.iter().map(|item| {
                            let post_profile = item.post_profile.clone();
                        
                            let topic = post_profile.topic.clone();
                            let id = post_profile.post_id;
                            let author_id = post_profile.author_id;
                            let author_pic_path = post_profile.author_profile.clone().and_then(|author| author.pic_path).unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_string());
                            let author_full_name = post_profile.author_profile.clone().and_then(|author| Some(author.full_name)).unwrap_or("".to_string());
                            // let archived = post_profile.post_group.clone().and_then(|post_group| Some(post_group.archived)).unwrap_or(false);
                            // let published = post_profile.post_group.clone().and_then(|post_group| Some(post_group.published)).unwrap_or(false);
    
                            let naive = chrono::NaiveDate::from_ymd_opt(2024, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();
    
                            let timestamp = post_profile.timestamp;
                            
                            let time_fn = get_creation_date(timestamp);
                            
                            let shares = post_profile.post_group.clone().and_then(|post_group| Some(post_group.message_posts)).unwrap_or(vec![]).len();                            
                            
                            let timestamp_published = post_profile.post_group.clone().and_then(|post_group| post_group.maybe_timestamp).unwrap_or(naive);
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
                                archived: false,
                                published: false,
                                on_dropdown_menu: false,
                            }
                        }).collect();

                        self.post_list = post_list;

                        for post_list in self.post_list.iter_mut() {
                            for item in post_group.iter() {
                                if post_list.post_id.0 == item.post_id {
                                    post_list.archived = item.archived;
                                    post_list.published = item.published;
                                }
                            }
                        }

                    }
                }

                ctx.link().send_message(PostsListMessage::ChangeFilter(self.filter.clone()));


            }
            PostsListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;

                let posts_clone = self.post_list.clone();

                let posts: Vec<PostProfile> = posts_clone.iter().filter(|filter| {
                    let archived = filter.archived.clone();
                    let published = filter.published.clone();

                    self.filter == PostFilter::Alls && archived == false ||
            
                    self.filter == PostFilter::Published && published == true && archived == false ||
    
                    self.filter == PostFilter::Unpublished && archived == false && published == false ||
    
                    self.filter == PostFilter::Archived && archived == true && published == false
                })
                .cloned()
                .collect();

                info!("FILTER {:?} <-----> POSTS - VIEW {:?} ", self.filter, posts);

                self.post_list_view = posts;
            }
            PostsListMessage::ShowDropdownFilter => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            PostsListMessage::CreatePost => {
                if let (Some(section_id), Some(graphql_task)) = (self.section_id, self.graphql_task.as_mut()) {
                    let local = Local::now().naive_local();

                    if let Some(inventory_group_id) = ctx.props().inventory_group {
                        let vars = post_model::post_class_and_group_create::Variables { 
                            topic: String::from(lang::dict("~ New Post ~")),
                            content: String::from(""),
                            group_id: ctx.props().group_id.0,
                            inventory_group_id,
                            timestamp: local,
                            post_id: Uuid::new_v4(),
                            section_id
                        };
    
                        let task = post_model::PostClassAndGroupCreate::request(
                            graphql_task,
                            &ctx,
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
            }
            PostsListMessage::RemovePost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = post_model::post_group_delete::Variables { 
                        group_id: ctx.props().group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostGroupDelete::request(
                        graphql_task,
                        &ctx,
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
                if let Some(graphql_task) = self.graphql_task.as_mut() {


                    let vars = post_model::delete_post::Variables { 
                        post_id: post_id.0,
                    };

                    let task = post_model::DeletePost::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
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
                info!("Remove Post {:?}", post_id);
                
            }
            PostsListMessage::AddPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = post_model::post_group_add::Variables { 
                        group_id: ctx.props().group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostGroupAdd::request(
                        graphql_task,
                        &ctx,
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
                let group_id = ctx.props().group_id;
                let school_id = ctx.props().school_id;

                if let Some(post_id) = post_id {
                    ctx.link().navigator().unwrap().push(&AppRoute::Post{school_id, group_id, post_id});
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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);
        
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let on_alls = ctx.link().callback(|_| PostsListMessage::ChangeFilter(PostFilter::Alls));
        let on_published = ctx.link().callback(|_| PostsListMessage::ChangeFilter(PostFilter::Published));
        let on_unpublished = ctx.link().callback(|_| PostsListMessage::ChangeFilter(PostFilter::Unpublished));
        let on_archived = ctx.link().callback(|_| PostsListMessage::ChangeFilter(PostFilter::Archived));
        let on_dropdown = ctx.link().callback(|_| PostsListMessage::ShowDropdownFilter);
        let on_change_list = ctx.link().callback(|(post_id, published, archived)| PostsListMessage::UpdatePostList(post_id, published, archived));

        let on_post_delete = ctx.link().callback(|post_id| PostsListMessage::RemovePost(post_id));
        let on_post_delete_entirely = ctx.link().callback(|post_id| PostsListMessage::RemovePostEntirely(post_id));

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

        let maybe_dropdown_by_user = ctx
            .props()
            .user_profile
            .clone()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick={on_alls}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_published}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Published {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_unpublished}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == PostFilter::Unpublished {true} else {false}} />
                                        <span class={if self.filter == PostFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_archived}>
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

        let posts_list = self.post_list_view.iter()
            .map(|item| {            

            html! {
                <PostCard 
                    group_id={ctx.props().group_id}
                    school_id={ctx.props().school_id}
                    on_post_delete={on_post_delete.clone()}
                    on_change_list={on_change_list.clone()}
                    topic={item.topic.clone()}
                    timestamp={item.timestamp.clone()}
                    timestamp_published={item.timestamp_published.clone()}
                    shares={item.shares}
                    post_id={item.post_id}
                    author_id={item.author_id}
                    author_pic_path={item.author_pic_path.clone()}
                    author_full_name={item.author_full_name.clone()}
                    archived={item.archived}
                    published={item.published}
                    user_profile={ctx.props().user_profile.clone()}
                    on_post_delete_entirely={on_post_delete_entirely.clone()} />
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

        let maybe_post_search = ctx
            .props()
            .user_profile
            .clone()
            .and_then(|item| {
                let on_select = ctx.link().callback(|select_option| match select_option {
                    PostSelectOption::Post(post_id) => PostsListMessage::AddPost(post_id)
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <PostSelect user_profile={ctx.props().user_profile.clone()}
                            on_select={on_select} 
                            allow_create={true}
                            school_id={ctx.props().school_id}
                            group_id={group_id.clone()} />
                    })
                } else {
                    Some(html! {
                        <SearchPostsGroup user_profile={ctx.props().user_profile.clone()}
                            group_id={ctx.props().group_id}
                            school_id={ctx.props().school_id}/>
                    })
                }
            })
            .unwrap_or(html! {});
        let maybe_user_profile_pic = ctx
            .props()
            .user_profile
            .clone()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });

        
        let navigator = ctx.link().navigator().unwrap();
        let on_direct_meet = Callback::from(move |_| navigator.push(&AppRoute::MeetDirect{group_id}));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {ctx.props().class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick={on_direct_meet}>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {maybe_post_search}
                {maybe_user_profile_pic}
            </div>
        };

        let maybe_new = ctx
            .props()
            .user_profile
            .clone()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    let on_select = ctx.link().callback(move |_| PostsListMessage::CreatePost);
                    Some(html! {
                        <>
                            <a class="button btn-create-card d-flex align-items-center justify-content-center" onmousedown={on_select.clone()}>
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
                    {lang::dict("Posts")} <span class="ps-1">{"("}{self.post_list_view.iter().cloned().len()}{")"}</span>
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