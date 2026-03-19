use log::*;
use uuid::Uuid;
use yew::prelude::*;
use std::time::Duration;
use gloo_storage::Storage;
use web_sys::{Node, window};
use yew::virtual_dom::VNode;
use code_location::code_location;
use yew::{html, Component, Html};
use gloo_timers::callback::Interval;
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::lesson_model;
use roboxmaker_models::lesson_model::lesson_by_id;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_searches::search_lesson_group::SearchLessonGroup;
use roboxmaker_message::{message_list::MessageList, MessageGroupCategory};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, SubscriptionTask, Subscribe};
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, SchoolId, ClassGroupCategory, UserId, MyUserProfile};

#[derive(Debug)]
pub enum LessonPageViewEdit {
    None,
    Edit,
    Done,
    Save,
}

pub struct LessonPageView {
    graphql_task: Option<GraphQLTask>,
    update_lesson_task: Option<RequestTask>,
    load_task: Option<SubscriptionTask>,
    save_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    lesson: Option<lesson_model::lesson_by_id::LessonByIdLessonGroupByPk>,
    node: Option<Node>,
    edit: LessonPageViewEdit,
    title: String,
    content: String,
    save_status: bool,
    job: Option<Interval>,
    on_dropdown_menu: bool,
    type_filter: bool,
    teaching_cards: bool,
    class_name: String,
    del_lesson_entirely_modal: bool,
    maybe_load_spinner: bool,
    saved_sidebar_state: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonPageViewProperties {
    // pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum LessonPageViewMessage {
    // AppRoute(AppRoute),
    StartAutoSave,
    StopAutoSave,
    FetchLessonById(LessonId, GroupId),
    Lesson(Option<lesson_model::lesson_by_id::ResponseData>),
    DeleteLessonById(LessonId),
    // LessonDeleted(Option<lesson_model::delete_lesson_by_id::ResponseData>),
    LessonDeleted(Option<lesson_model::lesson_group_delete::ResponseData>),
    DeleteLessonEntirely(LessonId),
    LessonDeletedEnt(Option<lesson_model::delete_lesson::ResponseData>),
    Edit(LessonPageViewEdit),
    Content(String),
    Title(String),
    Saved(Option<lesson_model::lesson_by_id_update::ResponseData>),
    Back,
    SendToGrade(Option<lesson_model::update_lesson_group_options::ResponseData>),
    // ArchivedToggle(Option<lesson_model::update_lesson_group_options::ResponseData>),
    NoSendLessonToGrade(LessonId),
    SendLessonToGrade(LessonId),
    // ArchivedLesson(LessonId),
    OnDropdownMenu,
    ChangeSidebarState,
    OnDeleteLessonEntirely,
}

impl Component for LessonPageView {
    type Message = LessonPageViewMessage;
    type Properties = LessonPageViewProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(LessonPageViewMessage::FetchLessonById(ctx.props().lesson_id, ctx.props().group_id));

        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        roboxmaker_utils::functions::school_state();

        LessonPageView {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_lesson_task: None,
            load_task: None,
            save_task: None,
            delete_task: None,
            lesson: None,
            node: None,
            edit: LessonPageViewEdit::None,
            title: String::from(""),
            content: String::from(""),
            save_status: true,
            job: None,
            on_dropdown_menu: false,
            type_filter: false,
            teaching_cards: false,
            class_name: String::from(""),
            del_lesson_entirely_modal: false,
            maybe_load_spinner: false,
            saved_sidebar_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            // LessonPageViewMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route);
            // }
            LessonPageViewMessage::StartAutoSave => {

                let duration = Duration::from_secs(600).as_secs() as u32;
                let link = ctx.link().clone();
                let handle = Interval::new( duration, move || {
                    link.send_message(LessonPageViewMessage::Edit(LessonPageViewEdit::Save))
                });
                self.job = Some(handle);
                should_update = true;
            }
            LessonPageViewMessage::StopAutoSave => {
                self.job = None;
            }
            LessonPageViewMessage::FetchLessonById(lesson_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::lesson_by_id::Variables { 
                        lesson_id: lesson_id.0,
                        group_id: group_id.0,
                    };

                    let task = lesson_model::LessonById::subscribe(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            LessonPageViewMessage::Lesson(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            LessonPageViewMessage::Lesson(response) => {
                self.lesson = response.clone().and_then(|data| data.lesson_group_by_pk);

                if let Some(lesson) = &self.lesson {

                    self.class_name = lesson.clone().class_profile.and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or("".to_string());

                    if let Some(lesson_profile) = &lesson.lesson_profile {
                        self.title = lesson_profile.title.clone();

                        if lesson_profile.lesson_type == Some(lesson_by_id::RoboxLessonTypeEnum::ElectronicsLessons) {
                            self.type_filter = true;
                        } else {
                            self.type_filter = false;
                        }

                        if lesson_profile.lesson_type == Some(lesson_by_id::RoboxLessonTypeEnum::TeachingCards) {
                            self.teaching_cards = true;
                        } else {
                            self.teaching_cards = false;
                        }
                    }

                    if let Some(lesson_content) = &lesson.lesson_content {
                        self.content = lesson_content.content.clone();
                        let node = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.create_element("div").ok())
                            .and_then(|div| {
                                div.set_class_name("ck-content");
                                div.set_inner_html(&lesson_content.content);
                                Some(Node::from(div))
                            });
                        self.node = node;
                    }
                    if response.is_some() {
                        self.maybe_load_spinner = false;
                    }
                }

                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = ctx.props().school_id;
                let group_id = ctx.props().group_id;
                let category = ClassGroupCategory::Lessons;
                if response.clone().and_then(|data| data.lesson_group_by_pk).is_none() {
                    if ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        ctx.link().navigator().unwrap().push(&AppRoute::SchoolGroupSection{school_id, group_id, category});
                    
                    } else {
                        
                        ctx.link().navigator().unwrap().push(&AppRoute::GroupSectionStudent{school_id, user_id, category});
                    }
                }
            }
            LessonPageViewMessage::DeleteLessonById(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::lesson_group_delete::Variables { 
                        group_id: ctx.props().group_id.0,
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::LessonGroupDelete::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            LessonPageViewMessage::LessonDeleted(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            LessonPageViewMessage::LessonDeleted(response) => {
                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = ctx.props().school_id;
                let group_id = ctx.props().group_id;
                let category = ClassGroupCategory::Lessons;

                if response.clone().and_then(|data| data.delete_lesson_group).is_some() {
                    if ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        ctx.link().navigator().unwrap().push(&AppRoute::SchoolGroupSection{school_id, group_id, category});
                    
                    } else {
                        
                        ctx.link().navigator().unwrap().push(&AppRoute::GroupSectionStudent{school_id, user_id, category});
                    }
                    
                    info!("{:?} del", response);
                }
                should_update = true;
            }
            LessonPageViewMessage::DeleteLessonEntirely(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::delete_lesson::Variables { 
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::DeleteLesson::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            LessonPageViewMessage::LessonDeletedEnt(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            LessonPageViewMessage::LessonDeletedEnt(response) => {
                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = ctx.props().school_id;
                let group_id = ctx.props().group_id;
                let category = ClassGroupCategory::Lessons;

                if response.clone().and_then(|data| data.delete_lesson_by_pk).is_some() {
                    if ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        ctx.link().navigator().unwrap().push(&AppRoute::SchoolGroupSection{school_id, group_id, category});
                    
                    } else {
                        
                        ctx.link().navigator().unwrap().push(&AppRoute::GroupSectionStudent{school_id, user_id, category});
                    }
                    
                    info!("{:?} del", response);
                }
            }
            LessonPageViewMessage::Title(title) => {
                self.title = title;
                self.save_status = false;
                should_update = true;
            }
            LessonPageViewMessage::Content(content) => {
                self.content = content;
                self.save_status = false;
                should_update = true;
            }
            LessonPageViewMessage::Edit(edit) => {
                self.edit = edit;
                match self.edit {
                    LessonPageViewEdit::Edit => {
                        ctx.link().send_message(LessonPageViewMessage::StartAutoSave);
                        self.edit = LessonPageViewEdit::Edit;
                    }
                    LessonPageViewEdit::None => {
                    }
                    LessonPageViewEdit::Done => {
                        self.save_status = true;

                        ctx.link().send_message(LessonPageViewMessage::StopAutoSave)
                    }
                    LessonPageViewEdit::Save => {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = lesson_model::lesson_by_id_update::Variables { 
                                lesson_id: ctx.props().lesson_id.0,
                                lesson_title: self.title.clone(),
                                lesson_content: self.content.clone(),
                            };
        
                            let task = lesson_model::LessonByIdUpdate::request(
                                graphql_task,
                                &ctx,
                                vars,
                                |response| {
                                    LessonPageViewMessage::Saved(response)
                                },
                            );
                            self.save_task = Some(task);
                        }

                        self.edit = LessonPageViewEdit::None;
                    }
                }
            }
            LessonPageViewMessage::Saved(response) => {
                if response.clone().and_then(|data| data.update_lesson_profile_by_pk).is_some() &&
                response.clone().and_then(|data| data.update_lesson_content_by_pk).is_some() {
                    self.save_status = true;
                }
            }
            LessonPageViewMessage::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            LessonPageViewMessage::SendToGrade(response) => {
                response
                    .and_then(|data| data.update_lesson_group_by_pk)
                    .clone().and_then(|update_lesson_group_by_pk| Some(update_lesson_group_by_pk.send_to_grade))
                    .unwrap_or(false);
            }
            // LessonPageViewMessage::ArchivedToggle(response) => {
            //     if let Some(mut lessons) = ctx.props().lessons.clone() {
            //         lessons.archived = response.and_then(|data| data.update_lesson_group_by_pk).clone().and_then(|update_lesson_group_by_pk| Some(update_lesson_group_by_pk.archived)).unwrap_or(false)
            //     }
            // }
            LessonPageViewMessage::SendLessonToGrade(lesson_id) => {
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
                        |response| {
                            LessonPageViewMessage::SendToGrade(response)
                        },
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            LessonPageViewMessage::NoSendLessonToGrade(lesson_id) => {
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
                        |response| {
                            LessonPageViewMessage::SendToGrade(response)
                        },
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            // LessonPageViewMessage::ArchivedLesson(lesson_id) => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = lesson_model::update_lesson_group_options::Variables { 
            //             lesson_id: lesson_id.0,
            //             group_id: ctx.props().group_id.0,
            //             archived: true,
            //             send_to_grade: false,
            //         };

            //         let task = lesson_model::UpdateLessonGroupOptions::request(
            //             graphql_task,
            //             &self.link,
            //             vars,
            //             |response| {
            //                 LessonPageViewMessage::ArchivedToggle(response)
            //             },
            //         );
            //         self.update_lesson_task = Some(task);
            //     }
            // }
            LessonPageViewMessage::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            LessonPageViewMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
            LessonPageViewMessage::OnDeleteLessonEntirely => {
                self.del_lesson_entirely_modal = !self.del_lesson_entirely_modal
            }
        };
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
        let on_show_del_lesson = ctx.link().callback(move |_| LessonPageViewMessage::OnDeleteLessonEntirely);

        let on_show_sidebar = ctx.link().callback(move |_| LessonPageViewMessage::ChangeSidebarState);

        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };
        let lesson_id = ctx.props().lesson_id;

        // let on_archived_lesson = ctx.link().callback(move |_| LessonPageViewMessage::ArchivedLesson(lesson_id));        
        let on_send_to_grade = ctx.link().callback(move |_| LessonPageViewMessage::SendLessonToGrade(lesson_id));
        let on_no_send_to_grade = ctx.link().callback(move |_| LessonPageViewMessage::NoSendLessonToGrade(lesson_id));

        let author_lesson = self
            .lesson
            .as_ref()
            .and_then(|lesson | lesson.lesson_profile.as_ref())
            .and_then(|lesson_profile| {
                let author_profile = lesson_profile.author_profile.as_ref().unwrap();
                let pic_path = author_profile.pic_path.clone().unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_owned());
                Some(html! {
                    <div class="d-flex flex-wrap align-items-center justify-content-between mb-6 mt-5">
                        <div class="d-flex align-items-center">
                            <img class="img-card-32" src={pic_path} alt="" />
                            <span class="text-dark noir-light is-size-18 lh-22 ps-2">{&author_profile.full_name}</span>
                        </div>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <i class="far fa-clock"></i>
                            <span class="ps-2">{&lesson_profile.timestamp.format("%a %e %b %Y %T").to_string()}</span>
                        </span>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <i class="fas fa-graduation-cap"></i>
                            <span class="ps-2">{self.class_name.clone()}</span>
                        </span>
                    </div>
                })
            })
            .unwrap_or(html! {});
        let status_save = if self.save_status {
            html! {
                <span class="text-success mx-4">{lang::dict("Saved")}</span>
            }
        } else {
            html! {
                <span class="text-danger mx-4">{lang::dict("Unsaved")}</span>
            }
        };
        if let Some(lesson) = &self.lesson {
            let id = lesson.clone().lesson_profile.and_then(|data| Some(data.lesson_id)).unwrap_or(Uuid::default());
            let lesson_id = LessonId(id);
            let on_del_lesson_entirely = ctx.link().callback(move |_| LessonPageViewMessage::DeleteLessonEntirely(lesson_id));

            let naive_default = chrono::NaiveDate::from_ymd_opt(2022, 01, 01).unwrap().and_hms_opt(01, 01, 01).unwrap();
            let naivedatetime = chrono::NaiveDate::from_ymd_opt(2023, 01, 05).unwrap().and_hms_opt(12, 01, 01).unwrap();
            let _timestamp_filter = if lesson.lesson_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive_default).gt(&naivedatetime) {
                true
            } else {
                false
            };

            let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

            let ahothor_id = lesson.clone().lesson_profile.and_then(|data| Some(data.author_id)).unwrap_or(Uuid::default());

            let author_valid = if ahothor_id == user_id.0 {
                true
            } else {
                false
            };
            let maybe_edit_options = match self.edit {
                LessonPageViewEdit::Edit => {
                    let on_done = ctx
                        .link()
                        .callback(move |_| LessonPageViewMessage::Edit(LessonPageViewEdit::Done));
                    let on_save = ctx
                        .link()
                        .callback(move |_| LessonPageViewMessage::Edit(LessonPageViewEdit::Save));

                    let on_data = ctx.link().callback(|data: InputEvent| LessonPageViewMessage::Title(get_value_from_input_event(data)));
                    
                    html! {
                        <>
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Lesson Title")} value={self.title.clone()} oninput={on_data} />
                            <a class="btn button-cancel-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick={on_done}>
                                <span class="text-white">
                                    <i class="fas fa-times fas fa-lg"></i>
                                </span>
                            </a>
                            {status_save}
                            <a class="btn button-save-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick={on_save}>
                                <span class="text-white">
                                    <i class="fas fa-cloud-upload-alt fas fa-lg"></i>
                                </span>
                            </a>
                        </>
                    }
                }
                _=> {
                    let maybe_lesson_edit = ctx
                        .props()
                        .user_profile
                        .as_ref()
                        .and_then(|item| {
                            let lesson_id = lesson.clone().lesson_profile.and_then(|data| Some(data.lesson_id)).unwrap_or(Uuid::default());
                            let on_edit = ctx
                                .link()
                                .callback( |_| LessonPageViewMessage::Edit(LessonPageViewEdit::Edit));
                            let on_delete = ctx
                                .link()
                                .callback( move |_| LessonPageViewMessage::DeleteLessonById(LessonId(lesson_id)));
                            let on_dropdown = ctx
                                .link()
                                .callback( move |_| LessonPageViewMessage::OnDropdownMenu);
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
                            let maybe_no_send = if lesson.send_to_grade {
                                html! {
                                    <li>
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_no_send_to_grade}>
                                            <i class="fas fa-upload me-2"></i>
                                            <span>{lang::dict("Do Not Post")}</span>
                                        </a>
                                    </li>
                                }
                            } else {
                                html! {}
                            };

                            let maybe_send = if lesson.send_to_grade {
                                html! {}
                            } else {
                                html! {
                                    <li>
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_send_to_grade}>
                                            <i class="fas fa-upload me-2"></i>
                                            <span>{lang::dict("To Post")}</span>
                                        </a>
                                    </li>
                                }
                            };

                            let spinner = if self.maybe_load_spinner {
                                html! {
                                    <div class="text-center text-gray-purple-two">
                                        <div class="spinner-border" role="status">
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            };

                            // if item.user_staff.is_some() || user_profile.user_teacher.is_some() || user_profile.id == lesson_profile.author_id {
                            if item.user_staff.is_some() {
                                Some(html! {
                                    <div class="dropdown">
                                        <a class={maybe_menu} onclick={on_dropdown} role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                            <i class="fas fa-ellipsis-v"></i>
                                        </a>
                                        <ul class={maybe_item} aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                            <li>
                                                <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_edit}>
                                                    <i class="fas fa-edit me-2"></i>
                                                    <span>{lang::dict("Edit")}</span>
                                                </a>
                                            </li>
                                            {
                                                if self.maybe_load_spinner {
                                                    {spinner}
                                                } else {
                                                    html! {
                                                        <>
                                                            {maybe_no_send}
                                                            {maybe_send}
                                                        </>
                                                    }
                                                }
                                            }
                                            <li class="border-top">
                                                <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={&on_delete}>
                                                    <i class="fas fa-lock me-2"></i>
                                                    <span>{lang::dict("Disguise")}</span>
                                                </a>
                                                <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_lesson}>
                                                    <i class="fas fa-trash me-2"></i>
                                                    <span>{lang::dict("Remove")}</span>
                                                </a>
                                            </li>
                                        </ul>
                                    </div>
                                })
                            } else if item.user_teacher.is_some() {
                                Some(html! {
                                    <div class="dropdown">
                                        <a class={maybe_menu} onclick={on_dropdown} role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                            <i class="fas fa-ellipsis-v"></i>
                                        </a>
                                        <ul class={maybe_item} aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                            {
                                                if self.teaching_cards == false && author_valid == true {
                                                    html! {
                                                        <>
                                                            <li>
                                                                <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_edit}>
                                                                    <i class="fas fa-edit me-2"></i>
                                                                    <span>{lang::dict("Edit")}</span>
                                                                </a>
                                                            </li>
                                                            {
                                                                if self.maybe_load_spinner {
                                                                    {spinner}
                                                                } else {
                                                                    html! {
                                                                        <>
                                                                            {maybe_no_send}
                                                                            {maybe_send}
                                                                        </>
                                                                    }
                                                                }
                                                            }
                                                            <li class="border-top">
                                                                <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_delete}>
                                                                    <i class="fas fa-lock me-2"></i>
                                                                    <span>{lang::dict("Disguise")}</span>
                                                                </a>
                                                                <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_lesson}>
                                                                    <i class="fas fa-trash me-2"></i>
                                                                    <span>{lang::dict("Remove")}</span>
                                                                </a>
                                                            </li>
                                                        </>
                                                    }
                                                } else if self.type_filter == true {
                                                    {
                                                        if self.maybe_load_spinner {
                                                            {spinner}
                                                        } else {
                                                            html! {
                                                                <>
                                                                    {maybe_no_send}
                                                                    {maybe_send}
                                                                </>
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    html! {}
                                                }
                                            }
                                        </ul>
                                    </div>
                                })
                            } else {
                                None
                            }
                        })
                        .unwrap_or(html! {});

                    html! {
                        <div class="d-flex justify-content-between align-items-center w-100">
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{&self.title}</h1>
                            {maybe_lesson_edit}
                        </div>
                    }
                }
            };
            let maybe_lesson_content = {
                let on_data = ctx
                    .link()
                    .callback(move |data| LessonPageViewMessage::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                match self.edit {
                    LessonPageViewEdit::Edit => {
                        html! {
                            <div class="mb-6" style="border: 1px solid #C8C1CD; border-radius: 10px;">
                                <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()}
                                    content={self.content.clone()}
                                    upload_url={upload_url}
                                    on_data={on_data} />
                            </div>
                        }
                    }
                    _ => {
                        html! {
                            <span class="text-dark noir-light is-size-18 lh-22">
                                {VNode::VRef(self.node.clone().unwrap())}
                            </span>
                        }
                    }
                }
            };
            let maybe_messages_right = html! {
                <MessageList user_profile={ctx.props().user_profile.clone() }
                    user_id={None}
                    group_category={MessageGroupCategory::Lessons(ctx.props().group_id, 
                    ctx.props().lesson_id)} />
            };
            let school_id = ctx.props().school_id;
            let group_id = ctx.props().group_id;
            let category = ClassGroupCategory::Lessons;
            let navigator = ctx.link().navigator().unwrap();

            let go_back_group = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            let go_back_grade = html! {
                <>
                    <a onclick={go_back_group}>
                        <span class="text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
                            <span class="icon">
                                <i class="fas fa-arrow-left"></i>
                            </span>
                            <span class="mx-2">{lang::dict("To Lessons")}</span>
                            {self.class_name.clone()}
                        </span>
                    </a>
                </>
            };
            let maybe_user_profile_pic = ctx
                .props()
                .user_profile
                .as_ref()
                .and_then(|user_profile| Some(user_profile.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                    })
                })
                .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
                });

            let class_right_sidebar = if self.saved_sidebar_state {
                "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
            } else {
                "d-none"
            };
            let class_sidebar_mobile = if self.saved_sidebar_state {
                "offcanvas offcanvas-end show bg-silver d-block d-sm-block d-md-block d-lg-none d-xl-none d-xxl-none"
            } else {
                "offcanvas offcanvas-end"
            };
            let style_sidebar_mobile = if self.saved_sidebar_state {
                "visibility: visible;"
            } else {
                "display: none;"
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

            html! {
                <>
                    <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                        <div class="d-flex flex-column w-100 ps-3 pt-3 ps-md-5 pt-md-5 ps-lg-7 pt-lg-7">
                            <div class="d-flex flex-wrap align-items-center justify-content-between mb-6">
                                {go_back_grade}
                                {btn_sidebar_show}
                            </div>
                            <div class="d-flex flex-wrap align-items-center justify-content-between mb-4">
                                {maybe_edit_options}
                            </div>
                            {author_lesson}
                            {maybe_lesson_content}
                        </div>
                    </div>
                    <div class={class_right_sidebar}>
                        <div class="d-flex flex-row align-items-center justify-content-between pb-6">
                            <div class="me-5">
                                <SearchLessonGroup group_id={ctx.props().group_id}
                                    lesson_id={ctx.props().lesson_id }
                                    school_id={ctx.props().school_id} />
                            </div>
                            {maybe_user_profile_pic.clone()}
                        </div>
                        <div class="mb-3">
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Notes and comments")}</span>
                        </div>
                        {maybe_messages_right.clone()}
                    </div>
                    <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                        <div class="offcanvas-header d-flex justify-content-end">
                            <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="offcanvas-body pt-0">
                            <div class="d-flex flex-row align-items-center justify-content-between pb-6">
                                <div class="me-5">
                                    <SearchLessonGroup group_id={ctx.props().group_id}
                                        lesson_id={ctx.props().lesson_id }
                                        school_id={ctx.props().school_id} />
                                </div>
                                {maybe_user_profile_pic.clone()}
                            </div>
                            <div class="mb-3">
                                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Notes and comments")}</span>
                            </div>
                            {maybe_messages_right.clone()}
                        </div>
                    </div>
                    {modal_del_lesson_entirely}
                </>
            }
        } else {
            html! {
                <div class="progress w-100">
                    <div class="progress-bar" role="progressbar" style="width: 25%;" aria-valuenow="100" aria-valuemin="0" aria-valuemax="100">{"100%"}</div>
                </div>
            }
        }
    }
}