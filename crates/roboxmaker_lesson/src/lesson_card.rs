use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use crate::lesson_list::LessonProfile;
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::lesson_model::{self, get_lesson_list};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_loaders::placeholders::card_lesson_list::CardLessonListPlaceholder;
use roboxmaker_types::types::{AppRoute, GroupId, LessonId, SchoolId, MyUserProfile};


pub struct LessonCard {
    graphql_task: Option<GraphQLTask>,
    update_lesson_task: Option<RequestTask>,
    on_dropdown_menu: bool,
    maybe_placeholder: bool,
    link_download: String,
    del_lesson_entirely_modal: bool,
    maybe_load_spinner: bool,
    modal_publish: bool,
    send_to_grade: bool,
    archived: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonCardProperties {
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub on_lesson_delete: Option<Callback<LessonId>>,
    pub on_del_lesson_entirely: Callback<LessonId>,
    pub on_change_list: Callback<(LessonId, bool, bool)>,
    pub lesson_profile: Option<LessonProfile>,
    pub archived: bool,
    pub send_to_grade: bool,
}

#[derive(Debug)]
pub enum LessonCardMessage {
    DeleteLessonGroup(LessonId),
    DeleteLessonEntirely(LessonId),
    SendToGrade(Option<lesson_model::update_lesson_group_options::ResponseData>),
    // ArchivedToggle(Option<lesson_model::update_lesson_group_options::ResponseData>),
    SendLessonToGrade(LessonId),
    // ArchivedLesson(LessonId),
    NoSendLessonToGrade(LessonId),
    OnDropdownMenu,
    HiddenPlaceholder,
    ModalPublish,
    OnDeleteLessonEntirely,
}

impl Component for LessonCard {
    type Message = LessonCardMessage;
    type Properties = LessonCardProperties;

    fn create(ctx: &Context<Self>) -> Self {

        let start = "<a href=\'";
        let end = "\'>".to_owned() + &ctx.props().lesson_profile.clone().and_then(|d| Some(d.title.clone())).unwrap_or("".to_string()) + "</a>";
        let content_props = ctx.props().lesson_profile.clone().and_then(|d| Some(d.content)).unwrap_or("".to_string());
        let content = content_props.trim_start_matches(start);
        let link_download = content.trim_end_matches(&end);

        // info!("DATACONTENT {:?}", link_download);
        let send_to_grade = ctx.props().send_to_grade;
        let archived = ctx.props().archived;
        LessonCard {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_lesson_task: None,
            on_dropdown_menu: false,
            maybe_placeholder: true,
            link_download: link_download.to_string(),
            del_lesson_entirely_modal: false,
            maybe_load_spinner: false,
            modal_publish: false,
            send_to_grade,
            archived,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            LessonCardMessage::DeleteLessonGroup(lesson_id) => {
                if let Some(on_lesson_delete) = &ctx.props().on_lesson_delete {
                    on_lesson_delete.emit(lesson_id)
                }
            }
            LessonCardMessage::DeleteLessonEntirely(lesson_id) => {
                ctx.props().on_del_lesson_entirely.emit(lesson_id)
            }
            LessonCardMessage::OnDeleteLessonEntirely => {
                self.del_lesson_entirely_modal = !self.del_lesson_entirely_modal
            }
            LessonCardMessage::SendToGrade(response) => {
                let lesson_id = ctx.props().lesson_id;
                if response
                    .clone()
                    .and_then(|data| data.update_lesson_group_by_pk)
                    .clone()
                    .and_then(|update_lesson_group_by_pk| {
                        Some(update_lesson_group_by_pk.send_to_grade)
                    })
                    .is_some()
                {
                    self.send_to_grade = response
                        .clone()
                        .and_then(|data| data.update_lesson_group_by_pk)
                        .clone()
                        .and_then(|update_lesson_group_by_pk| {
                            Some(update_lesson_group_by_pk.send_to_grade)
                        })
                        .unwrap_or(false);
                    self.archived = response
                        .clone()
                        .and_then(|data| data.update_lesson_group_by_pk)
                        .clone()
                        .and_then(|update_lesson_group_by_pk| {
                            Some(update_lesson_group_by_pk.archived)
                        })
                        .unwrap_or(false);
                    ctx.props().on_change_list.emit((
                        lesson_id,
                        ctx.props().send_to_grade,
                        ctx.props().archived,
                    ));

                    self.maybe_load_spinner = false;
                    self.modal_publish = false;
                }
            }
            LessonCardMessage::SendLessonToGrade(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::update_lesson_group_options::Variables {
                        lesson_id: lesson_id.0,
                        group_id: ctx.props().group_id.0,
                        archived: false,
                        send_to_grade: true,
                    };

                    let task = lesson_model::UpdateLessonGroupOptions::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| LessonCardMessage::SendToGrade(response),
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            LessonCardMessage::NoSendLessonToGrade(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::update_lesson_group_options::Variables {
                        lesson_id: lesson_id.0,
                        group_id: ctx.props().group_id.0,
                        archived: false,
                        send_to_grade: false,
                    };

                    let task = lesson_model::UpdateLessonGroupOptions::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| LessonCardMessage::SendToGrade(response),
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            LessonCardMessage::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            LessonCardMessage::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
            LessonCardMessage::ModalPublish => {
                self.modal_publish = !self.modal_publish
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props() != old_props {
            should_render = true;
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let group_id = ctx.props().group_id;
        let lesson_id = ctx.props().lesson_id;
        let school_id = ctx.props().school_id;

        // let on_archived_lesson = self
        //     .link
        //     .callback(move |_| LessonCardMessage::ArchivedLesson(lesson_id));

        let on_send_to_grade = ctx
            .link()
            .callback(move |_| LessonCardMessage::SendLessonToGrade(lesson_id));
        let on_no_send_to_grade = ctx
            .link()
            .callback(move |_| LessonCardMessage::NoSendLessonToGrade(lesson_id));

        let navigator = ctx.link().navigator().unwrap();
        let on_lesson = Callback::from(move |_| navigator.push(&AppRoute::Lesson{school_id, group_id, lesson_id}));
        
        let navigator = ctx.link().navigator().unwrap();
        let on_lesson_view = Callback::from(move |_| navigator.push(&AppRoute::LessonView{school_id, group_id, lesson_id}));
        // let on_lesson = ctx.link().callback(move |_| {
        //     LessonCardMessage::AppRoute(AppRoute::Lesson{school_id, group_id, lesson_id})
        // });
        // let on_lesson_view = ctx.link().callback(move |_| {
        //     LessonCardMessage::AppRoute(AppRoute::LessonView{school_id, group_id, lesson_id})
        // });


        let on_lesson_delete = ctx
            .link()
            .callback(move |_| LessonCardMessage::DeleteLessonGroup(lesson_id));

        let on_del_lesson_entirely = ctx.link().callback(move |_| LessonCardMessage::DeleteLessonEntirely(lesson_id));

        let on_show_del_lesson = ctx.link().callback(move |_| LessonCardMessage::OnDeleteLessonEntirely);

        let on_dropdown = ctx
            .link()
            .callback(move |_| LessonCardMessage::OnDropdownMenu);
        let maybe_menu = if self.on_dropdown_menu {
            "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0 show"
        } else {
            "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0"
        };
        let maybe_item = if self.on_dropdown_menu {
            "dropdown-menu show"
        } else {
            "dropdown-menu"
        };

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

        let send_to_grade_btn = if ctx.props().send_to_grade {
            html! {
                <li class="my-1">
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&on_no_send_to_grade}>
                        <img class="me-2" src="/icons/upload-2.svg" style="height: 25px;" />
                        <span>{lang::dict("Do Not Post")}</span>
                    </a>
                </li>
            }
        } else {
            html! {
                <li class="my-1">
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&on_send_to_grade}>
                        <img class="me-2" src="/icons/upload-2.svg" style="height: 25px;" />
                        <span>{lang::dict("To Post")}</span>
                    </a>
                </li>
            }
        };

        let maybe_dropdown = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <a class={maybe_menu} onclick={on_dropdown} role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                <i class="fas fa-ellipsis-v"></i>
                            </a>
                            <ul class={maybe_item} aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                <li class="my-1">   
                                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&on_lesson}>
                                        <i class="fas fa-edit fas fa-lg me-2 ms-1"></i>
                                        <span>{lang::dict("Edit")}</span>
                                    </a>
                                </li>
                                {
                                    if self.maybe_load_spinner {
                                        {spinner}
                                    } else {
                                        {send_to_grade_btn}
                                    }
                                }
                                <li class="border-top">
                                    <a class="dropdown-item drop-hover-filter text-purple-gray my-2" onclick={&on_lesson_delete}>
                                        <i class="fas fa-lock fas fa-lg me-2 ms-1"></i>
                                        <span>{lang::dict("Disguise")}</span>
                                    </a>
                                    <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_lesson}>
                                        <img class="me-2" src="/icons/trash.svg" style="height: 22px;" />
                                        <span>{lang::dict("Remove")}</span>
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


        let archived_lesson = if ctx.props().archived {
            html! {
                <div class="saved-archived-container d-flex align-items-center justify-content-center ms-1">
                    <span class="text-white noir-bold is-size-12 lh-14">{lang::dict("Filed")}</span>
                </div>
            }
        } else {
            html! {}
        };

        let maybe_title_lesson = if ctx.props().archived {
            "title-lesson-archived text-primary-blue-dark noir-bold is-size-18 lh-22"
        } else {
            "title-lesson text-primary-blue-dark noir-bold is-size-18 lh-22"
        };

        let on_show_modal_publish = ctx.link().callback(move |_| LessonCardMessage::ModalPublish);

        let send_no_to_grade_btn = if ctx.props().send_to_grade {
            html! {
                <div class="d-flex is-justify-content-end">
                    <a onclick={&on_show_modal_publish}>
                        <span class="text-gray-blue">
                            <i class="fas fa-lock"></i>
                        </span>
                    </a>
                </div>
            }
        } else {
            html! {}
        };

        let send_to_grade_btn = if ctx.props().send_to_grade {
            html! {}
        } else {
            html! {
                <div class="d-flex is-justify-content-end">
                    <a onclick={&on_show_modal_publish}>
                        <span class="text-gray-blue">
                            <i class="fas fa-share"></i>
                        </span>
                    </a>
                </div>
            }
        };

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
                            if ctx.props().send_to_grade {
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
                        <span class="text-primary-blue-dark noir-medium is-size-18 lh-22">{&ctx.props().lesson_profile.clone().and_then(|d| Some(d.title.clone())).unwrap_or("".to_string())}</span>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={&on_show_modal_publish} data-bs-dismiss="modal">{"Cancelar"}</button>
                        {   
                            if ctx.props().send_to_grade {
                                html! {}
                            } else {
                                html! {
                                    <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_send_to_grade}>{"Publicar"}</button>
                                }
                            }
                        }
                        {   
                            if ctx.props().send_to_grade {
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

        let class_del_show = if self.del_lesson_entirely_modal {
            "modal fade show"
        } else {
            "modal fade"
        };

        let style_del_display = if self.del_lesson_entirely_modal {
            "display: block;"
        } else {
            "display: none;"
        };

        let modal_del_lesson_entirely = if self.del_lesson_entirely_modal {
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
                            <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_del_lesson_entirely}>{"Confirmar"}</button>
                        </div>
                    </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        let maybe_user_option = ctx.props().user_profile.clone().and_then(|item| {
            if item.user_staff.is_some() || item.user_teacher.is_some() {
                Some(html! {
                    <>
                        {send_no_to_grade_btn}
                        {send_to_grade_btn}
                    </>
                })
            } else {
                None
            }

        }).unwrap_or(html! {});

        let maybe_lessons = ctx.props().lesson_profile.clone().and_then(|item| {

            let maybe_title_lesson_view = ctx.props().user_profile.clone().and_then(|user| {
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <span class={maybe_title_lesson}>{&item.title}</span>
                    })
                } else {
                    Some(html! {
                        <span class="title-lesson text-primary-blue-dark noir-bold is-size-18 lh-22" style="width: 245px !important;">{&item.title}</span>
                    })
                }
    
            }).unwrap_or(html! {});
    

            let maybe_option_menu = if item.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards {
                if item.author_id {
                    html! {
                        <>
                            <a onclick={&on_lesson_view} style="height: 29px;">
                                <div class="d-flex flex-wrap">
                                    {maybe_title_lesson_view.clone()}
                                    {archived_lesson.clone()}
                                </div>
                            </a>
                            <div class="d-flex is-justify-content-end">
                                <a href={self.link_download.clone()} target="_blank">
                                    <img src="/icons/download.svg" style="height: 22px;" />
                                </a>
                            </div>
                        </>
                    }
                } else {
                    html! {
                        <>
                            <div style="height: 29px;">
                                <div class="d-flex flex-wrap">
                                    {maybe_title_lesson_view.clone()}
                                    {archived_lesson.clone()}
                                </div>
                            </div>
                            <div class="d-flex is-justify-content-end">
                                <a href={self.link_download.clone()} target="_blank">
                                    <img src="/icons/download.svg" style="height: 22px;" />
                                </a>
                            </div>
                        </>
                    }                
                }
            } else if item.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons {
                html! {
                    <>
                        <a onclick={&on_lesson_view} style="height: 29px;">
                            <div class="d-flex flex-wrap">
                                {maybe_title_lesson_view.clone()}
                                // <span class=maybe_title_lesson>{&ctx.props().title}</span>
                                {archived_lesson.clone()}
                            </div>
                        </a>
                        {maybe_user_option}
                    </>
                }
            } else {
                html! {
                    <>
                        <a onclick={&on_lesson_view} style="height: 29px;">
                            <div class="d-flex flex-wrap">
                                // <span class=maybe_title_lesson>{&ctx.props().title}</span>
                                {maybe_title_lesson_view.clone()}
                                {archived_lesson.clone()}
                            </div>
                        </a>
                        {maybe_dropdown}
                    </>
                }
            };

            let maybe_times = if item.lesson_type == get_lesson_list::RoboxLessonTypeEnum::TeachingCards {
                {
                    if item.author_id {
                        html! {
                            <a onclick={&on_lesson_view}>
                                <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                                    <div class="d-flex align-items-center">
                                        <a onclick={&on_lesson_view}>
                                            <img class="img-card-32" src={item.school_logo.clone()} />
                                        </a>
                                        <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-1">{&item.school_name}</span>
                                    </div>
                                    <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                        <span class="icon me-1">
                                            <i class="far fa-clock"></i>
                                        </span>
                                        <span class="d-flex flex-column">
                                            <span>{&item.timestamp}</span>
                                        </span>
                                    </span>
                                </div>
                            </a>
                        }
                    } else {
                        html! {
                            <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                                <div class="d-flex align-items-center">
                                    <img class="img-card-32" src={item.school_logo.clone()} />
                                    <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-1">{&item.school_name}</span>
                                </div>
                                <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                    <span class="icon me-1">
                                        <i class="far fa-clock"></i>
                                    </span>
                                    <span class="d-flex flex-column">
                                        <span>{&item.timestamp}</span>
                                    </span>
                                </span>
                            </div>
                        }
                    }
                }
            } else if item.lesson_type == get_lesson_list::RoboxLessonTypeEnum::ElectronicsLessons {
                html! {
                    <a onclick={&on_lesson_view}>
                        <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                            <div class="d-flex align-items-center">
                                <a onclick={&on_lesson_view}>
                                    <img class="img-card-32" src={item.school_logo.clone()} />
                                </a>
                                <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-1">{&item.school_name}</span>
                            </div>
                            <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                <span class="icon me-1">
                                    <i class="far fa-clock"></i>
                                </span>
                                <span class="d-flex flex-column">
                                    <span>{&item.timestamp}</span>
                                </span>
                            </span>
                        </div>
                    </a>
                }
            } else {
                html! {
                    <a onclick={&on_lesson_view}>
                        <div class="d-flex d-flex-align-items-center justify-content-between pb-4 px-4">
                            <div class="d-flex align-items-center">
                                <a onclick={&on_lesson_view}>
                                    <img class="img-card-32" src={item.author_pic_path.clone()} />
                                </a>
                                <span class="author-lesson text-dark noir-light is-size-14 lh-18 ps-2">{&item.author_full_name}</span>
                            </div>
                            <span class="text-gray-purple-two noir-light is-size-14 lh-17 d-flex align-items-center">
                                <span class="icon me-1">
                                    <i class="far fa-clock"></i>
                                </span>
                                <span class="d-flex flex-column">
                                    <span>{&item.timestamp}</span>
                                </span>
                            </span>
                        </div>
                    </a>
                }
            };
    
            Some(html! {
                <div class="card-lesson bg-white d-flex flex-column justify-content-between mb-3 mb-lg-5 me-md-3 me-lg-5">
                    <div class="d-flex is-align-content-flex-start justify-content-between pt-4 px-4">
                        {maybe_option_menu}
                    </div>
                    {maybe_times}
                </div>
            })
        }).unwrap_or(html! {
            <CardLessonListPlaceholder />
        });

        html! {
            <>
                {modal_del_lesson_entirely}
                {modal_view}
                {maybe_lessons}
            </>
        }
    }
}
