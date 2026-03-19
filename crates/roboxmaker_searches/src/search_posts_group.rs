use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_main::lang;
use roboxmaker_models::post_model;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{PostId, GroupId, AppRoute, LoadSearch, LoadSearchFound, SchoolId, MyUserProfile};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, Clone, PartialEq)]
pub enum PostMode {
    Edit,
    Preview
}

pub struct SearchPostsGroup {
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    posts_by_grade: Option<post_model::search_by_post_grade_by_group_id::SearchByPostGradeByGroupIdGroupByPk>,
    search_node: NodeRef,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchPostsGroupProperties {
    pub user_profile: Option<MyUserProfile>,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum SearchPostsGroupMessage {
    // AppRoute(AppRoute),
    FetchPostsByPostsGrade(String),
    PostsByGrade(Option<post_model::search_by_post_grade_by_group_id::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchPostsGroup {
    type Message = SearchPostsGroupMessage;
    type Properties = SearchPostsGroupProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        SearchPostsGroup {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            posts_by_grade: None,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = true;
        match msg {
            // SearchPostsGroupMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route)
            // }
            SearchPostsGroupMessage::FetchPostsByPostsGrade(search) => {
                should_update = false;
                let group_id = ctx.props().group_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::search_by_post_grade_by_group_id::Variables { 
                        search: format!("%{}%", search), 
                        group_id: group_id.0 
                    };

                    let task = post_model::SearchByPostGradeByGroupId::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            SearchPostsGroupMessage::PostsByGrade(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            SearchPostsGroupMessage::PostsByGrade(response) => {
                self.posts_by_grade = response.clone().and_then(|data| data.group_by_pk);

                if !response.clone().and_then(|data| data.group_by_pk).clone().and_then(|group_by_pk| Some(group_by_pk.post_groups)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            SearchPostsGroupMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchPostsGroupMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.maybe_section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            SearchPostsGroupMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.list_search_state = LoadSearch::Static;
            }
        };
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let on_focus = ctx.link().callback(move |_| SearchPostsGroupMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| SearchPostsGroupMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| SearchPostsGroupMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| SearchPostsGroupMessage::FetchPostsByPostsGrade(get_value_from_input_event(search)));

        let posts_by_grade = self
            .posts_by_grade.clone().and_then(|data| Some(data.post_groups))
            .unwrap_or(vec![])
            .iter()
            .zip(ctx
                    .props().user_profile
                    .as_ref()
                )
            .map(|(data, user)| {
                let post_uuid = data.post_profile.clone().and_then(|data| Some(data.post_id)).unwrap_or(Uuid::default());
                let topic = data.post_profile.clone().and_then(|data| Some(data.topic)).unwrap_or("".to_string());
                let post_id = PostId(post_uuid);
                let school_id = ctx.props().school_id;
                let navigator = ctx.link().navigator().unwrap();
                let navigator_two = ctx.link().navigator().unwrap();
                let on_post_edit = Callback::from(move |_| navigator.push(&AppRoute::Post{school_id, group_id, post_id}));
                let on_post_view = Callback::from(move |_| navigator_two.push(&AppRoute::PostView{school_id, group_id, post_id}));
        
                // let on_post_edit = ctx.link().callback(move |_| SearchPostsGroupMessage::AppRoute(AppRoute::Post{school_id, group_id, post_id}));  
                // let on_post_view = ctx.link().callback(move |_| SearchPostsGroupMessage::AppRoute(AppRoute::PostView{school_id, group_id, post_id})); 
                let maybe_user = if user.user_staff.is_some() || user.user_teacher.is_some() {
                    html! {
                        <>
                            <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_post_edit}>
                                <span>
                                    {lang::dict("Edit")}
                                </span>
                            </a>
                            <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_post_view}>
                                <span>
                                    {lang::dict("View")}
                                </span>
                            </a>
                        </>
                    }
                } else {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_post_view}>
                            <span>
                                {lang::dict("View")}
                            </span>
                        </a>
                    }
                };
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&topic}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex flex-wrap px-5 py-2">
                                {maybe_user}
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        let maybe_message_response = match self.list_search_state {
            LoadSearch::Static => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Posts")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ posts_by_grade }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Posts")}</span>
                    </div>
                }
            },
        };
        let class_search_modal = if self.maybe_section_search {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.maybe_section_search {
            "display: block;"
        } else {
            "display: none;"
        };
        html! {
            <>
                <a class="button-search-univeral mt-3" onclick={&on_hidden_modal}>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
                    </span>
                </a>
                <div class={class_search_modal} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_search_scroll} aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class" ref={self.search_node.clone()}
                                        oninput={on_search.clone()} onfocus={on_focus.clone()} onblur={on_blur.clone()} placeholder={lang::dict("Search")} />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick={&on_hidden_modal}>
                                    <span class="text-white">
                                        <i class="fas fa-times"></i>
                                    </span>
                                </a>
                            </div>
                            <div class="modal-body vh-100">
                                {maybe_message_response}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}