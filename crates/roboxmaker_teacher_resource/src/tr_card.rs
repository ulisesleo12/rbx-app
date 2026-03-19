use log::*;
use yew::prelude::*;
use std::time::Duration;
use crate::tr_list::TRProfile;
use yew::format::{Json, Nothing};
use code_location::code_location;
use yew::services::{FetchService, Task, TimeoutService};
use yew::services::fetch::{FetchTask, Response, Request};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, teacher_resource};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request as OtherRequest, RequestTask};
use roboxmaker_loaders::placeholders::card_lesson_list::CardLessonListPlaceholder;
use roboxmaker_types::types::{AppRoute, GroupId, MyUserProfile, ResourceId, SchoolId};


pub struct TeacherResourceCard {
    link: ComponentLink<Self>,
    props: TeacherResourceCardProperties,
    graphql_task: Option<GraphQLTask>,
    update_tr_task: Option<RequestTask>,
    file_task: Option<FetchTask>,
    on_dropdown_menu: bool,
    maybe_placeholder: bool,
    job: Option<Box<dyn Task>>,
    modal_delete_resource_by_id: bool,
    load_spinner: bool,
    modal_publish: bool,
    resource_link: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct TeacherResourceCardProperties {
    pub resource_id: ResourceId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Option<Callback<AppRoute>>,
    pub on_delete_resource: Option<Callback<ResourceId>>,
    pub on_delete_resource_by_id: Callback<ResourceId>,
    pub on_change_list: Callback<(ResourceId, bool, bool)>,
    pub tr_profile: TRProfile,
    pub archived: bool,
    pub send_to_grade: bool,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    ResourceGroupDelete(ResourceId),
    DeleteResourceById(ResourceId),
    RespPublish(Option<teacher_resource::update_teacher_resource_group_options::ResponseData>),
    Publish(ResourceId),
    UnPublish(ResourceId),
    OnDropdownMenu,
    HiddenPlaceholder,
    ModalPublish,
    ShowModalDeleteResourceById,

    GetResourceFiles,
    ResourceFilesResp(Vec<String>),
}

impl Component for TeacherResourceCard {
    type Message = Message;
    type Properties = TeacherResourceCardProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let handle = TimeoutService::spawn(
            Duration::from_millis(400),
            link.callback(|_| Message::HiddenPlaceholder),
        );

        link.send_message(Message::GetResourceFiles);

        TeacherResourceCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_tr_task: None,
            file_task: None,
            on_dropdown_menu: false,
            maybe_placeholder: true,
            job: Some(Box::new(handle)),
            modal_delete_resource_by_id: false,
            load_spinner: false,
            modal_publish: false,
            resource_link: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        match msg {
            Message::AppRoute(route) => {
                if let Some(on_app_route) = &self.props.on_app_route {
                    on_app_route.emit(route);
                }
            }
            Message::ResourceGroupDelete(resource_id) => {
                if let Some(on_delete_resource) = &self.props.on_delete_resource {
                    on_delete_resource.emit(resource_id);
                    
                    self.on_dropdown_menu = false;
                }
            }
            Message::DeleteResourceById(resource_id) => {
                self.props.on_delete_resource_by_id.emit(resource_id);

                self.on_dropdown_menu = false;
                self.modal_delete_resource_by_id = false;
            }
            Message::ShowModalDeleteResourceById => {
                self.modal_delete_resource_by_id = !self.modal_delete_resource_by_id
            }
            Message::RespPublish(response) => {
                let resource_id = self.props.resource_id;
                if response
                    .clone()
                    .and_then(|data| data.update_teacher_resource_group_by_pk)
                    .clone()
                    .and_then(|update_teacher_resource_group_by_pk| {
                        Some(update_teacher_resource_group_by_pk.send_to_grade)
                    })
                    .is_some()
                {
                    self.props.send_to_grade = response
                        .clone()
                        .and_then(|data| data.update_teacher_resource_group_by_pk)
                        .clone()
                        .and_then(|update_teacher_resource_group_by_pk| {
                            Some(update_teacher_resource_group_by_pk.send_to_grade)
                        })
                        .unwrap_or(false);
                    self.props.archived = response
                        .clone()
                        .and_then(|data| data.update_teacher_resource_group_by_pk)
                        .clone()
                        .and_then(|update_teacher_resource_group_by_pk| {
                            Some(update_teacher_resource_group_by_pk.archived)
                        })
                        .unwrap_or(false);

                    self.props.on_change_list.emit((
                        resource_id,
                        self.props.send_to_grade,
                        self.props.archived,
                    ));

                    self.load_spinner = false;
                    self.modal_publish = false;
                }
            }
            Message::Publish(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::update_teacher_resource_group_options::Variables {
                        resource_id: resource_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: true,
                    };

                    let task = teacher_resource::UpdateTeacherResourceGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::RespPublish(response),
                    );
                    self.update_tr_task = Some(task);
                    self.load_spinner = true;
                }
            }
            Message::UnPublish(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::update_teacher_resource_group_options::Variables {
                        resource_id: resource_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: false,
                    };

                    let task = teacher_resource::UpdateTeacherResourceGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::RespPublish(response),
                    );
                    self.update_tr_task = Some(task);
                    self.load_spinner = true;
                }
            }
            Message::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            Message::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
            Message::ModalPublish => {
                self.modal_publish = !self.modal_publish
            }
            Message::GetResourceFiles => {
                let url = format!("https://files.roboxmaker.com/file-res.php?file={}*", self.props.resource_id.0);

                let get_request = Request::get(url)
                    .header("robox-resource-id", self.props.resource_id.0.to_string())
                    .body(Nothing)
                    .expect("Unable to build request!");
    
                let get_callback = self.link.callback(move |response: Response<Json<Result<Vec<String>, anyhow::Error>>>| {
                    let (_meta, Json(files)) = response.into_parts();

                    if let Ok(files) = files {
                        Message::ResourceFilesResp(files)
                    } else {
                        Message::ResourceFilesResp(vec![])
                    }
                });
        
                let task = FetchService::fetch(get_request, get_callback);
                self.file_task = task.ok();
            }
            Message::ResourceFilesResp(files) => {
                if let Some(file) = files.first() {
                    self.resource_link = file.to_string()
                }
            }
        }
        true
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
        let _none = self.job.as_ref();

        let group_id = self.props.group_id;
        let resource_id = self.props.resource_id;
        let school_id = self.props.school_id;

        let on_send_to_grade = self
            .link
            .callback(move |_| Message::Publish(resource_id));
        let on_no_send_to_grade = self
            .link
            .callback(move |_| Message::UnPublish(resource_id));
        let on_lesson = self.link.callback(move |_| {
            Message::AppRoute(AppRoute::Resource(school_id, group_id, resource_id))
        });

        let on_del_tr_entirely = self.link.callback(move |_| Message::DeleteResourceById(resource_id));
        let on_show_del_lesson = self.link.callback(move |_| Message::ShowModalDeleteResourceById);

        let archived_resource = if self.props.archived {
            html! {
                <div class="saved-archived-container d-flex align-items-center justify-content-center ms-1">
                    <span class="text-white noir-bold is-size-12 lh-14">{lang::dict("Filed")}</span>
                </div>
            }
        } else {
            html! {}
        };

        let maybe_title_resource = if self.props.archived {
            "title-lesson-archived text-primary-blue-dark noir-bold is-size-18 lh-22"
        } else {
            "title-lesson text-primary-blue-dark noir-bold is-size-18 lh-22"
        };

        let on_show_modal_publish = self.link.callback(move |_| Message::ModalPublish);

        let class_show = if self.modal_publish {
            "modal fade show"
        } else {
            "modal fade"
        };

        let style_display = if self.modal_publish {
            "display: block;"
        } else {
            "display: none;"
        };

        let modal_view = html! {
            <div class={class_show} style={style_display} id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                <div class="modal-dialog">
                <div class="modal-content">
                    <div class="modal-header d-flex justify-content-center">
                        {   
                            if self.props.send_to_grade {
                                html! {
                                    <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"No Publicar La Lección"}</h1>
                                }
                            } else {
                                html! {
                                    <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"Publicar La Lección"}</h1>
                                }
                            }
                        }
                    </div>
                    <div class="modal-body text-center">
                        <span class="text-primary-blue-dark noir-medium is-size-18 lh-22">{&self.props.tr_profile.title}</span>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={&on_show_modal_publish} data-bs-dismiss="modal">{"Cancelar"}</button>
                        {   
                            if self.props.send_to_grade {
                                html! {}
                            } else {
                                html! {
                                    <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_send_to_grade}>{"Publicar"}</button>
                                }
                            }
                        }
                        {   
                            if self.props.send_to_grade {
                                html! {
                                    <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_no_send_to_grade}>{"No Publicar"}</button>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
                </div>
            </div>
        };

        let class_del_show = if self.modal_delete_resource_by_id {
            "modal fade show"
        } else {
            "modal fade"
        };

        let style_del_display = if self.modal_delete_resource_by_id {
            "display: block;"
        } else {
            "display: none;"
        };

        let modal_delete_resource_by_id = if self.modal_delete_resource_by_id {
            html! {
                <div class={class_del_show} style={style_del_display} id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                    <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header d-flex justify-content-center">
                            <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"Borrar lección"}</h1>
                        </div>
                        <div class="modal-body text-center">
                            <span class="text-primary-blue-dark noir-medium is-size-16 lh-22">{"Para borrar la lección por completo, presione "}
                                <span class="noir-bold">{"confirmar"}</span></span>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={&on_show_del_lesson} data-bs-dismiss="modal">{"Cancelar"}</button>
                            <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_del_tr_entirely}>{"Confirmar"}</button>
                        </div>
                    </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        let maybe_title_resource_view = self.props.user_profile.clone().and_then(|item| {
            if item.user_staff.is_some() || item.user_teacher.is_some() {
                Some(html! {
                    <span class=maybe_title_resource>{&self.props.tr_profile.title}</span>
                })
            } else {
                None
            }

        }).unwrap_or(html! {});

        let action_menu = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {

                let maybe_menu = if self.on_dropdown_menu {
                    "btn btn-outline-secondary dropdown-toggle menu-hidden-toggle show border-0"
                } else {
                    "btn btn-outline-secondary dropdown-toggle menu-hidden-toggle border-0"
                };
                let maybe_item = if self.on_dropdown_menu {
                    "dropdown-menu show"
                } else {
                    "dropdown-menu"
                };
                // let maybe_no_send = if self.props.tr_profile.send_to_degree {
                //     html! {
                //         <li>
                //             <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={ self.link.callback(move |_| Message::UnPublish(resource_id)) }>
                //                 <i class="fas fa-upload me-2"></i>
                //                 <span>{lang::dict("Do Not Post")}</span>
                //             </a>
                //         </li>
                //     }
                // } else {
                //     html! {}
                // };

                // let maybe_send = if self.props.tr_profile.send_to_degree {
                //     html! {}
                // } else {
                //     html! {
                //         <li>
                //             <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={ self.link.callback(move |_| Message::Publish(resource_id)) }>
                //                 <i class="fas fa-upload me-2"></i>
                //                 <span>{lang::dict("To Post")}</span>
                //             </a>
                //         </li>
                //     }
                // };

                // let spinner = if self.load_spinner {
                //     html! {
                //         <div class="text-center text-gray-purple-two">
                //             <div class="spinner-border" role="status">
                //             </div>
                //         </div>
                //     }
                // } else {
                //     html! {}
                // };

                if item.user_staff.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <a class=maybe_menu onclick={ self.link.callback( move |_| Message::OnDropdownMenu) } role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                <i class="fas fa-ellipsis-v"></i>
                            </a>
                            <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                <li class="my-1">   
                                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&on_lesson}>
                                        <i class="fas fa-edit fas fa-lg me-2 ms-1"></i>
                                        <span>{lang::dict("Edit")}</span>
                                    </a>
                                </li>
                                // {
                                //     if self.load_spinner {
                                //         {spinner}
                                //     } else {
                                //         html! {
                                //             <>
                                //                 {maybe_no_send}
                                //                 {maybe_send}
                                //             </>
                                //         }
                                //     }
                                // }
                                <li class="border-top">
                                    <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={ self.link.callback( move |_| Message::ResourceGroupDelete(resource_id)) }>
                                        <i class="fas fa-lock me-2"></i>
                                        <span>{lang::dict("Disguise")}</span>
                                    </a>
                                    <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={ self.link.callback(move |_| Message::ShowModalDeleteResourceById) }>
                                        <i class="fas fa-trash me-2"></i>
                                        <span>{lang::dict("Remove")}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else if !self.resource_link.is_empty() {
                    Some(html! {
                        <div class="d-flex is-justify-content-end">
                            <a href={ format!("https://files.roboxmaker.com/resources/{}", self.resource_link.clone()) } target="_blank">
                                <img src="/icons/download.svg" style="height: 22px;" />
                            </a>
                        </div>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let lessons = if self.maybe_placeholder {
            html! {
                <CardLessonListPlaceholder />
            }
        } else {
            html! {
                <div class="card-lesson bg-white d-flex flex-column justify-content-between mb-3 mb-lg-5 me-md-3 me-lg-5">
                    <div class="d-flex is-align-content-flex-start justify-content-between pt-4 px-4">
                        {
                            if let Some(user_profile) = &self.props.user_profile {
                                if user_profile.user_staff.is_some() {
                                    html! {
                                        <a onclick={&on_lesson} style="height: 29px;">
                                            <div class="d-flex flex-wrap">
                                                {maybe_title_resource_view.clone()}
                                                {archived_resource.clone()}
                                            </div>
                                        </a>
                                    }
                                } else if !self.resource_link.is_empty() {
                                    html! {
                                        <a href={ format!("https://files.roboxmaker.com/resources/{}", self.resource_link.clone()) } target="_blank" style="height: 29px;">
                                            <div class="d-flex flex-wrap">
                                                {maybe_title_resource_view.clone()}
                                                {archived_resource.clone()}
                                            </div>
                                        </a>
                                    }
                                } else { html! {
                                    <a style="height: 29px;">
                                        <div class="d-flex flex-wrap">
                                            {maybe_title_resource_view.clone()}
                                            {archived_resource.clone()}
                                        </div>
                                    </a>
                                } }
                            } else { html! {} }
                        }
                        { action_menu }
                    </div>
                    {
                        if let Some(user_profile) = &self.props.user_profile {
                            if user_profile.user_staff.is_some() {
                                html! {
                                    <a onclick=&on_lesson>
                                        <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                                            <div class="d-flex align-items-center">
                                                <a>
                                                    <img class="img-card-32" src={self.props.tr_profile.author_pic_path.clone()} />
                                                </a>
                                                <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-2">{&self.props.tr_profile.author_full_name}</span>
                                            </div>
                                            <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                                <span class="icon me-1">
                                                    <i class="far fa-clock"></i>
                                                </span>
                                                <span class="d-flex flex-column">
                                                    <span>{&self.props.tr_profile.timestamp}</span>
                                                </span>
                                            </span>
                                        </div>
                                    </a>
                                }
                            } else if !self.resource_link.is_empty() {
                                html! {
                                    <a href={ format!("https://files.roboxmaker.com/resources/{}", self.resource_link.clone()) } target="_blank">
                                        <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                                            <div class="d-flex align-items-center">
                                                <a>
                                                    <img class="img-card-32" src={self.props.tr_profile.author_pic_path.clone()} />
                                                </a>
                                                <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-2">{&self.props.tr_profile.author_full_name}</span>
                                            </div>
                                            <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                                <span class="icon me-1">
                                                    <i class="far fa-clock"></i>
                                                </span>
                                                <span class="d-flex flex-column">
                                                    <span>{&self.props.tr_profile.timestamp}</span>
                                                </span>
                                            </span>
                                        </div>
                                    </a>
                                }
                            } else { html! {
                                <a>
                                    <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                                        <div class="d-flex align-items-center">
                                            <a>
                                                <img class="img-card-32" src={self.props.tr_profile.author_pic_path.clone()} />
                                            </a>
                                            <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-2">{&self.props.tr_profile.author_full_name}</span>
                                        </div>
                                        <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                            <span class="icon me-1">
                                                <i class="far fa-clock"></i>
                                            </span>
                                            <span class="d-flex flex-column">
                                                <span>{&self.props.tr_profile.timestamp}</span>
                                            </span>
                                        </span>
                                    </div>
                                </a>
                            } }
                        } else { html! {} }
                    }
                </div>
            }
        };

        html! {
            <>
                { modal_delete_resource_by_id }
                { modal_view }
                { lessons }
            </>
        }
    }
}
