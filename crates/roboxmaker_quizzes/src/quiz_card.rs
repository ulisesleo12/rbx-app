use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::quizzes_list::QuizProfile;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


use roboxmaker_main::lang;
use roboxmaker_models::quiz_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, MyUserProfile};


pub struct QuizCard {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    update_post_task: Option<RequestTask>,
    del_post_entirely_modal: bool,
    maybe_load_spinner: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub on_quiz_delete: Option<Callback<Uuid>>,
    pub on_quiz_delete_entirely: Callback<Uuid>,
    pub on_change_list: Callback<(Uuid, bool, bool)>,
    pub quiz_profile: QuizProfile
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    DeletePost(Uuid),
    DeletePostEntirely(Uuid),
    SaveDraftToggle(Option<quiz_model::update_quiz_group_options::ResponseData>),
    ArchivedToggle(Option<quiz_model::update_quiz_group_options::ResponseData>),
    ArchivedPost(Uuid),
    PublishedPost(Uuid),
    NoPublishedPost(Uuid),
    OnDropdownMenu,
    OnDeletePostEntirely,
}

impl Component for QuizCard {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QuizCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_post_task: None,
            maybe_load_spinner: false,
            del_post_entirely_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            Message::DeletePost(quiz_id) => {
                if let Some(on_quiz_delete) = &self.props.on_quiz_delete {
                    on_quiz_delete.emit(quiz_id);
                }
            }
            Message::DeletePostEntirely(quiz_id) => {
                self.props.on_quiz_delete_entirely.emit(quiz_id);
                
                self.del_post_entirely_modal = false;
            }
            Message::OnDeletePostEntirely => {
                self.del_post_entirely_modal = !self.del_post_entirely_modal;
            }
            Message::SaveDraftToggle(response) => {
                let quiz_id = self.props.quiz_profile.quiz_id;
                if response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.published)).is_some() {

                    self.props.quiz_profile.published = response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
                    self.props.quiz_profile.archived = response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.archived)).unwrap_or(false);
                    self.props.on_change_list.emit((quiz_id, self.props.quiz_profile.published, self.props.quiz_profile.archived));

                    self.maybe_load_spinner = false;
                }
            }
            Message::ArchivedToggle(response) => {
                let quiz_id = self.props.quiz_profile.quiz_id;
                if response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.archived)).is_some() {

                    self.props.quiz_profile.published = response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
                    self.props.quiz_profile.archived = response.clone().and_then(|data| data.update_quizzes_group_by_pk).clone().and_then(|data| Some(data.archived)).unwrap_or(false);
                    self.props.on_change_list.emit((quiz_id, self.props.quiz_profile.published, self.props.quiz_profile.archived));
                    
                    self.maybe_load_spinner = false;
                }
            }
            Message::ArchivedPost(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::update_quiz_group_options::Variables { 
                        group_id: self.props.group_id.0,
                        quiz_id: quiz_id,
                        published: false,
                        archived: true,
                    };
                    let task = quiz_model::UpdateQuizGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::ArchivedToggle(response)
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            Message::PublishedPost(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::update_quiz_group_options::Variables { 
                        group_id: self.props.group_id.0,
                        quiz_id: quiz_id,
                        published: true,
                        archived: false,
                    };
                    let task = quiz_model::UpdateQuizGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::ArchivedToggle(response)
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            Message::NoPublishedPost(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::update_quiz_group_options::Variables { 
                        group_id: self.props.group_id.0,
                        quiz_id: quiz_id,
                        published: false,
                        archived: false,
                    };
                    let task = quiz_model::UpdateQuizGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::ArchivedToggle(response)
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            Message::OnDropdownMenu => {
                self.props.quiz_profile.on_dropdown_menu = !self.props.quiz_profile.on_dropdown_menu;
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
        let quiz_id = self.props.quiz_profile.quiz_id;

        let on_archived_post = self.link.callback(move |_| Message::ArchivedPost(quiz_id));        
        let on_published_post = self.link.callback(move |_| Message::PublishedPost(quiz_id)); 
        let on_not_published_post = self.link.callback(move |_| Message::NoPublishedPost(quiz_id));
        
        let on_show_del_post = self.link.callback(move |_| Message::OnDeletePostEntirely);
        let on_del_post_entirely = self.link.callback(move |_| Message::DeletePostEntirely(quiz_id));
        
        let school_id = self.props.school_id;
        let on_post = self
            .link
            .callback(move |_| Message::AppRoute(AppRoute::Quizzes(school_id, group_id, quiz_id)));

        let maybe_post_delete = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_quiz_delete = self
                    .link
                    .callback(move |_| Message::DeletePost(quiz_id));
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <li class="my-1">
                                <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={ &on_post }>
                                    <i class="fas fa-edit fas fa-lg me-2 ms-1"></i>
                                    <span>{lang::dict("Edit")}</span>
                                </a>
                            </li>
                            <li class="border-top">
                                <a class="dropdown-item drop-hover-filter text-purple-gray my-2" onclick={ &on_quiz_delete }>
                                    <i class="fas fa-lock fas fa-lg me-2 ms-1"></i>
                                    <span>{ lang::dict("Disguise") }</span>
                                </a>
                                <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={ &on_show_del_post }>
                                    <img class="me-2" src="/icons/trash.svg" style="height: 22px;" />
                                    <span>{ lang::dict("Remove") }</span>
                                </a>
                            </li>
                        </>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {}); 
        
        let published_draft = if self.props.quiz_profile.published {
            html! {}
        } else {
            html! {
                <div class="saved-draft-container d-flex align-items-center justify-content-center ms-2">
                    <span class="text-white noir-bold is-size-12 lh-14">{ lang::dict("Draft Copy") }</span>
                </div>
            }
        };

        let option_icon_text = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    let no_maybe_published_icon_text = if self.props.quiz_profile.published {
                        html! {
                            <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                                <i class="far fa-eye me-1"></i>
                                <span>{ lang::dict("Published") }</span>
                            </span>
                        }
                    } else {
                        html! {}
                    };
                    let maybe_published_icon_text = if self.props.quiz_profile.published {
                        html! {}
                    } else {
                        html! {
                            <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                                <i class="far fa-eye-slash me-1"></i>
                                <span>{lang::dict("Not published")}</span>
                            </span>
                        }
                    };
                    Some(html! {
                        <>
                            { no_maybe_published_icon_text }
                            { maybe_published_icon_text }
                        </>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let spinner = if self.maybe_load_spinner {
            html! {
                <div class="text-center text-purple-gray">
                    <div class="spinner-border" role="status">
                        // <span class="visually-hidden">Loading...</span>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        let published_option_btn = if self.props.quiz_profile.published {
            html! {
                <li class="my-1">   
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick=on_not_published_post>
                        <img class="me-2" src="/icons/upload.svg" style="height: 25px;" />
                        <span>{lang::dict("Do Not Post")}</span>
                    </a>
                </li>
            }
        } else {
            html! {                        
                <li class="my-1">   
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick=on_published_post>
                        <img class="me-2" src="/icons/upload.svg" style="height: 25px;" />
                        <span>{lang::dict("To Post")}</span>
                    </a>
                </li>
            }
        };

        let dropdown_menu = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                let on_dropdown = self
                    .link
                    .callback( move |_| Message::OnDropdownMenu);
                let maybe_menu = if self.props.quiz_profile.on_dropdown_menu {
                    "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0 show"
                } else {
                    "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0"
                };
                let maybe_item = if self.props.quiz_profile.on_dropdown_menu {
                    "dropdown-menu show"
                } else {
                    "dropdown-menu"
                };
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <a class={ maybe_menu } onclick={ on_dropdown } role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                <i class="fas fa-ellipsis-v"></i>
                            </a>
                            <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                {
                                    if self.maybe_load_spinner {
                                        { spinner }
                                    } else {
                                        { published_option_btn }
                                    }
                                }
                                <li class="my-1">
                                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={ on_archived_post }>
                                        <img class="me-2" src="/icons/archive.svg" style="height: 25px;" />
                                        <span>{ lang::dict("File") }</span>
                                    </a>
                                </li>
                                { maybe_post_delete }
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let class_del_show = if self.del_post_entirely_modal {
            "modal fade show"
        } else {
            "modal fade"
        };

        let style_del_display = if self.del_post_entirely_modal {
            "display: block;"
        } else {
            "display: none;"
        };

        let modal_del_post_entirely = if self.del_post_entirely_modal {
            html! {
                <div class={ class_del_show } style={ style_del_display } id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                    <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header d-flex justify-content-center">
                            <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"Borrar Publicación"}</h1>
                        </div>
                        <div class="modal-body text-center">
                            <span class="text-primary-blue-dark noir-medium is-size-16 lh-22">{"Para borrar la publicacón por completo, presione "}
                                <span class="noir-bold">{"confirmar"}</span></span>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={ &on_show_del_post } data-bs-dismiss="modal">{"Cancelar"}</button>
                            <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={ &on_del_post_entirely }>{"Confirmar"}</button>
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
                <div class="card-post-view bg-white d-flex flex-column justify-content-between p-4 mb-4 w-100" key={ quiz_id.to_string() }>
                    <div class="d-flex align-items-center justify-content-between">
                        <a onclick={ &on_post }>
                            <div class="d-flex flex-wrap">
                                <div class="module-message-universal-2 line-clamp-message-universal">
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{ &self.props.quiz_profile.title }</span>
                                </div>
                                { published_draft }
                            </div>
                        </a>
                        { dropdown_menu }
                    </div>
                    <a class="d-flex flex-wrap align-items-center justify-content-between" onclick={ &on_post }>
                        <div class="d-flex flex-row align-items-center justify-content-start col-6 col-sm-6 col-md-2 col-lg-3">
                            <i class="fas fa-list-ol fa-lg text-dark me-3"></i>
                            <span class="text-dark noir-light is-size-14 lh-17 ms-1">{ "Evaluación" }</span>
                        </div>
                        <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                            <i class="far fa-clock me-1"></i>
                            <span>{ &self.props.quiz_profile.created_at }</span>
                        </span>
                        { option_icon_text }
                    </a>
                </div>
                { modal_del_post_entirely }
            </>
        }
    }
}