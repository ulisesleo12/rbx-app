use log::*;
use uuid::Uuid;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, post_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{AppRoute, GroupId, MyUserProfile, PageMode, PostId, SchoolId};


#[derive(Debug, Clone)]
enum LoadSearchFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadSearch {
    Static,
    Load(LoadSearchFound),
}

#[derive(Debug)]
pub enum PostSelectOption {
    Post(PostId),
}

pub struct PostSelect {
    link: ComponentLink<Self>,
    props: PostSelectProperties,
    graphql_task: Option<GraphQLTask>,
    post_task: Option<RequestTask>,
    posts: Vec<post_model::posts_by_name::PostsByNamePost>,
    search_node: NodeRef,
    show_create: bool,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
    // quizzes: Vec<post_model::posts_by_name::PostsByNamePostProfile>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostSelectProperties {
    pub on_select: Callback<PostSelectOption>,
    pub allow_create: bool,
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum PostSelectMessage {
    AppRoute(AppRoute),
    FetchPostsByPostName(String),
    Posts(Option<post_model::posts_by_name::ResponseData>),
    SelectPost(PostSelectOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for PostSelect {
    type Message = PostSelectMessage;
    type Properties = PostSelectProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        PostSelect {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            post_task: None,
            posts: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
            // quizzes: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            PostSelectMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            PostSelectMessage::FetchPostsByPostName(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::posts_by_name::Variables { 
                        search
                    };

                    let task = post_model::PostsByName::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostSelectMessage::Posts(response)
                        },
                    );
                    self.post_task = Some(task);
                }
            }
            PostSelectMessage::Posts(posts) => {
                self.posts = posts.clone().and_then(|data| Some(data.post)).unwrap_or(vec![]);
                // self.quizzes = posts.clone().and_then(|data| Some(data.post_profile)).unwrap_or(vec![]);

                if !posts.clone().and_then(|data| Some(data.post)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
                // if !posts.clone().and_then(|data| Some(data.post_profile)).unwrap_or(vec![]).is_empty() {
                //     self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                // } else {
                //     self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                // }
            }

            PostSelectMessage::SelectPost(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.maybe_section_search = false;
                self.posts = vec![];
                // self.quizzes = vec![];
                self.props.on_select.emit(select_option);
            }
            PostSelectMessage::OnFocus => {
                self.show_create = true;
                self.maybe_section_search = true;
            }
            PostSelectMessage::OnBlur => {
                self.show_create = false;
                // self.maybe_section_search = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.posts = vec![];
                self.list_search_state = LoadSearch::Static;
            }
            PostSelectMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.posts = vec![];
                // self.quizzes = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let on_focus = self.link.callback(move |_| PostSelectMessage::OnFocus);
        let on_blur = self.link.callback(move |_| PostSelectMessage::OnBlur);
        let on_hidden_modal = self.link.callback(move |_| PostSelectMessage::HiddenModal);
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 0 {
                format!("%{}%", search.value)
            } else {
                format!("%{}%", search.value)
                // search.value
            };
            PostSelectMessage::FetchPostsByPostName(search)
        });

        let group_id = self.props.group_id;
        
        let posts = self
            .posts
            .iter()
            // .zip(self.quizzes.clone())
            // .map(|(post, quizzes)| {
            .map(|post| {
                let post_id = PostId(post.id);
                let school_id = self.props.school_id;

                let on_select = self.link.callback(move |_| {
                    PostSelectMessage::SelectPost(PostSelectOption::Post(post_id))
                });

                let on_post = self.link.callback(move |_| PostSelectMessage::AppRoute(AppRoute::Post(school_id, group_id, post_id, PageMode::View)));
                let topic = post.post_profile.clone().unwrap().topic;
                let group_uuid = post.post_profile.clone().and_then(|data| data.post_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Uuid::default());

                let post_option_btn = if self.props.group_id == GroupId(group_uuid) {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_post }>
                            <span>
                                {lang::dict("View")}
                            </span>
                        </a>
                    }
                } else {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_select }>
                            <span>
                                {lang::dict("Add")}
                            </span>
                        </a>
                    }
                };
                // let quiz_title = quizzes.maybe_quiz.clone().and_then(|item| item.title).unwrap_or_default();
                // let quiz_group_id = quizzes.post_group.clone().and_then(|item| Some(item.group_id)).unwrap_or_default();
                // let quiz_post_id = quizzes.post_id;

                // let on_select_quiz = self.link.callback(move |_| {
                //     PostSelectMessage::SelectPost(PostSelectOption::Post(PostId(quiz_post_id)))
                // });

                // let on_post_quiz = self.link.callback(move |_| PostSelectMessage::AppRoute(AppRoute::Post(school_id, group_id, PostId(quiz_post_id), PageMode::View)));
                // let quiz_option_btn = if self.props.group_id == GroupId(quiz_group_id) {
                //     html! {
                //         <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_post_quiz }>
                //             <span>
                //                 {lang::dict("View")}
                //             </span>
                //         </a>
                //     }
                // } else {
                //     html! {
                //         <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_select_quiz }>
                //             <span>
                //                 {lang::dict("Add")}
                //             </span>
                //         </a>
                //     }
                // };
                // if quizzes.post_id != post.id {
                // if post.id == quiz_post_id {
                    // html! {
                    //     <div class="m-4">
                    //         <div class="card card-search-u vh-15">
                    //             <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                    //                 <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                    //                     { &quiz_title }
                    //                 </span>
                    //             </div>
                    //             <div class="card-body border-top d-flex px-5 py-2">
                    //                 { quiz_option_btn }
                    //             </div>
                    //         </div>
                    //     </div>
                    // }
                    html! {
                        <div class="m-4">
                            <div class="card card-search-u vh-15">
                                <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                        { &topic }
                                    </span>
                                </div>
                                <div class="card-body border-top d-flex px-5 py-2">
                                    { post_option_btn }
                                </div>
                            </div>
                        </div>
                    }
                // } else {
                //     html! {}
                // }
            })
            .collect::<Html>();
        // let _quizzes = self
        //     .quizzes
        //     .iter()
        //     // .zip(self.posts.clone())
        //     // .map(|(quizzes, post)| {
        //     .map(|quizzes| {
        //         let quiz_post_id = quizzes.post_id;
        //         let school_id = self.props.school_id;
        //         let quiz_title = quizzes.maybe_quiz.clone().and_then(|item| item.title).unwrap_or_default();
        //         let quiz_group_id = quizzes.post_group.clone().and_then(|item| Some(item.group_id)).unwrap_or_default();

        //         let on_select_quiz = self.link.callback(move |_| {
        //             PostSelectMessage::SelectPost(PostSelectOption::Post(PostId(quiz_post_id)))
        //         });

        //         let on_post_quiz = self.link.callback(move |_| PostSelectMessage::AppRoute(AppRoute::Post(school_id, group_id, PostId(quiz_post_id), PageMode::View)));
        //         let quiz_option_btn = if self.props.group_id == GroupId(quiz_group_id) {
        //             html! {
        //                 <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_post_quiz }>
        //                     <span>
        //                         {lang::dict("View")}
        //                     </span>
        //                 </a>
        //             }
        //         } else {
        //             html! {
        //                 <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={ on_select_quiz }>
        //                     <span>
        //                         {lang::dict("Add")}
        //                     </span>
        //                 </a>
        //             }
        //         };
        //         // if quizzes.post_id == post.id {
        //             html! {
        //                 <div class="m-4">
        //                     <div class="card card-search-u vh-15">
        //                         <div class="module-message-universal line-clamp-message-universal p-2 h-80">
        //                             <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
        //                                 { &quiz_title }
        //                             </span>
        //                         </div>
        //                         <div class="card-body border-top d-flex px-5 py-2">
        //                             { quiz_option_btn }
        //                         </div>
        //                     </div>
        //                 </div>
        //             }
        //         // } else {
        //         //     html! {}
        //         // }
        //     })
        //     .collect::<Html>();
        let _maybe_option_user = self.props.user_profile.as_ref().and_then(|item| {
            if item.user_staff.is_some() || item.user_teacher.is_some() {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Todas las Publicaciones"}</span>
                })
            } else {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Publicaciones del grupo"}</span>
                })
            }
        }).unwrap_or(html! {});

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
                    <>
                        <div class="d-flex flex-wrap justify-content-center">{ posts }</div>
                        // <div class="d-flex flex-wrap justify-content-center">{ quizzes }</div>
                    </>
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
                <a class="button-search-univeral mt-3" onclick=&on_hidden_modal>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
                    </span>
                </a>
                <div class=class_search_modal id="exampleModalScrollable" tabindex="-1"
                    aria-labelledby="exampleModalScrollableTitle" style=class_search_scroll aria-modal="true"
                    role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class"
                                        ref=self.search_node.clone() oninput=on_search.clone() onfocus=on_focus.clone()
                                        onblur=on_blur.clone() placeholder=lang::dict("Search") />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick=&on_hidden_modal>
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
