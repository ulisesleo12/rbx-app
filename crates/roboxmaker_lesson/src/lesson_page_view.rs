use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::services::Task;
use std::time::Duration;
use yew::web_sys::window;
use gloo_storage::Storage;
use yew::virtual_dom::VNode;
use yew::web_sys::{Node, self};
use code_location::code_location;
use yew::services::interval::IntervalService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::lesson_model::lesson_by_id;
use roboxmaker_models::{school_model, lesson_model};
use roboxmaker_searches::search_lesson_group::SearchLessonGroup;
use roboxmaker_message::{message_list::MessageList, MessageGroupCategory};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, SubscriptionTask, Subscribe};
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, ClassGroupLesson, SchoolId, ClassGroupCategory, UserId, MyUserProfile};


#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    TeachingCards,
    ElectronicsLessons,
    Extra,
}

#[derive(Debug)]
pub enum LessonPageViewEdit {
    None,
    Edit,
    Done,
    Save,
}

pub struct LessonPageView {
    link: ComponentLink<Self>,
    props: LessonPageViewProperties,
    graphql_task: Option<GraphQLTask>,
    update_lesson_task: Option<RequestTask>,
    load_task: Option<SubscriptionTask>,
    save_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    get_lesson_type_task: Option<RequestTask>,
    lesson: Option<lesson_model::lesson_by_id::LessonByIdLessonGroupByPk>,
    node: Option<Node>,
    edit: LessonPageViewEdit,
    title: String,
    content: String,
    save_status: bool,
    job: Option<Box<dyn Task>>,
    on_dropdown_menu: bool,
    type_filter: bool,
    teaching_cards: bool,
    class_name: String,
    del_lesson_entirely_modal: bool,
    maybe_load_spinner: bool,
    robox_lesson_type: Vec<LessonType>,
    show_lesson_type_dropdown: bool,
    lesson_type: Option<LessonType>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonPageViewProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub lessons: Option<ClassGroupLesson>,
    pub saved_sidebar_state: bool,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    StartAutoSave,
    StopAutoSave,
    FetchLessonById(LessonId, GroupId),
    Lesson(Option<lesson_model::lesson_by_id::ResponseData>),
    DeleteLessonById(LessonId),
    // LessonDeleted(Option<lesson_model::delete_lesson_by_id::ResponseData>),
    LessonDeleted(Option<lesson_model::lesson_group_delete::ResponseData>),
    DeleteLessonEntirely(LessonId),
    LessonDeletedEnt(Option<lesson_model::delete_lesson_by_id::ResponseData>),
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

    GetLessonType,
    RespLessonType(Option<lesson_model::get_lesson_type::ResponseData>),
    ShowLessonTypeDropdown,
    SelectLessonType(Option<LessonType>),
}

impl Component for LessonPageView {
    type Message = Message;
    type Properties = LessonPageViewProperties;

    fn create(mut props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::GetLessonType);
        link.send_message(Message::FetchLessonById(props.lesson_id, props.group_id));

        props.saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };
        LessonPageView {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_lesson_task: None,
            load_task: None,
            save_task: None,
            delete_task: None,
            get_lesson_type_task: None,
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
            robox_lesson_type: vec![],
            show_lesson_type_dropdown: false,
            lesson_type: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            Message::StartAutoSave => {
                let handle = IntervalService::spawn(
                    Duration::from_secs(600),
                    self.link
                        .callback(|_| Message::Edit(LessonPageViewEdit::Save)),
                );
                self.job = Some(Box::new(handle));
                should_update = true;
            }
            Message::StopAutoSave => {
                self.job = None;
            }
            Message::FetchLessonById(lesson_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::lesson_by_id::Variables { 
                        lesson_id: lesson_id.0,
                        group_id: group_id.0,
                    };

                    let task = lesson_model::LessonById::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::Lesson(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            Message::Lesson(response) => {
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

                        if let Some(lesson_type) = &lesson_profile.lesson_type {
                            let lesson_type = match lesson_type {
                                lesson_by_id::RoboxLessonTypeEnum::ElectronicsLessons => LessonType::ElectronicsLessons,
                                lesson_by_id::RoboxLessonTypeEnum::Extra => LessonType::Extra,
                                lesson_by_id::RoboxLessonTypeEnum::TeachingCards => LessonType::TeachingCards,
                                lesson_by_id::RoboxLessonTypeEnum::Other(_) => LessonType::Extra,
                            };

                            self.lesson_type = Some(lesson_type)
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

                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.lesson_group_by_pk).is_none() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Lessons)));
                    }
                }
            }
            Message::DeleteLessonById(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::lesson_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::LessonGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::LessonDeleted(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            Message::LessonDeleted(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.delete_lesson_group).is_some() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Lessons)));
                    }
                    
                    info!("{:?} del", response);
                }
                should_update = true;
            }
            Message::DeleteLessonEntirely(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::delete_lesson_by_id::Variables { 
                        lesson_id: lesson_id.0,
                    };

                    let task = lesson_model::DeleteLessonById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::LessonDeletedEnt(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            Message::LessonDeletedEnt(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.delete_lesson_by_pk).is_some() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Lessons)));
                    }
                    
                    info!("{:?} del", response);
                }
            }
            Message::Title(title) => {
                self.title = title;
                self.save_status = false;
                should_update = true;
            }
            Message::Content(content) => {
                self.content = content;
                self.save_status = false;
                should_update = true;
            }
            Message::Edit(edit) => {
                self.edit = edit;
                match self.edit {
                    LessonPageViewEdit::Edit => {
                        self.link.send_message(Message::StartAutoSave);
                        self.edit = LessonPageViewEdit::Edit;
                    }
                    LessonPageViewEdit::None => {
                    }
                    LessonPageViewEdit::Done => {
                        self.save_status = true;

                        self.link.send_message(Message::StopAutoSave)
                    }
                    LessonPageViewEdit::Save => {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let lesson_type = match self.lesson_type.clone().unwrap_or(LessonType::Extra) {
                                LessonType::TeachingCards => lesson_model::lesson_by_id_update::RoboxLessonTypeEnum::TeachingCards,
                                LessonType::ElectronicsLessons => lesson_model::lesson_by_id_update::RoboxLessonTypeEnum::ElectronicsLessons,
                                LessonType::Extra => lesson_model::lesson_by_id_update::RoboxLessonTypeEnum::Extra,
                            };
                            let vars = lesson_model::lesson_by_id_update::Variables { 
                                lesson_id: self.props.lesson_id.0,
                                lesson_title: self.title.clone(),
                                lesson_content: self.content.clone(),
                                lesson_type,
                            };
        
                            let task = lesson_model::LessonByIdUpdate::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    Message::Saved(response)
                                },
                            );
                            self.save_task = Some(task);
                        }

                        self.edit = LessonPageViewEdit::None;
                    }
                }
            }
            Message::Saved(response) => {
                if response.clone().and_then(|data| data.update_lesson_profile_by_pk).is_some() &&
                response.clone().and_then(|data| data.update_lesson_content_by_pk).is_some() {
                    self.save_status = true;
                }
            }
            Message::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            Message::SendToGrade(response) => {
                if let Some(mut lessons) = self.props.lessons.clone() {
                    lessons.send_to_grade = response
                        .and_then(|data| data.update_lesson_group_by_pk)
                        .clone().and_then(|update_lesson_group_by_pk| Some(update_lesson_group_by_pk.send_to_grade))
                        .unwrap_or(false);
                }
            }
            // Message::ArchivedToggle(response) => {
            //     if let Some(mut lessons) = self.props.lessons.clone() {
            //         lessons.archived = response.and_then(|data| data.update_lesson_group_by_pk).clone().and_then(|update_lesson_group_by_pk| Some(update_lesson_group_by_pk.archived)).unwrap_or(false)
            //     }
            // }
            Message::SendLessonToGrade(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::update_lesson_group_options::Variables { 
                        lesson_id: lesson_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: true,
                    };

                    let task = lesson_model::UpdateLessonGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::SendToGrade(response)
                        },
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            Message::NoSendLessonToGrade(lesson_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::update_lesson_group_options::Variables { 
                        lesson_id: lesson_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: false,
                    };

                    let task = lesson_model::UpdateLessonGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::SendToGrade(response)
                        },
                    );
                    self.update_lesson_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            // Message::ArchivedLesson(lesson_id) => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = lesson_model::update_lesson_group_options::Variables { 
            //             lesson_id: lesson_id.0,
            //             group_id: self.props.group_id.0,
            //             archived: true,
            //             send_to_grade: false,
            //         };

            //         let task = lesson_model::UpdateLessonGroupOptions::request(
            //             graphql_task,
            //             &self.link,
            //             vars,
            //             |response| {
            //                 Message::ArchivedToggle(response)
            //             },
            //         );
            //         self.update_lesson_task = Some(task);
            //     }
            // }
            Message::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            Message::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.props.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.props.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.props.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
            Message::OnDeleteLessonEntirely => {
                self.del_lesson_entirely_modal = !self.del_lesson_entirely_modal
            }
            Message::GetLessonType => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::get_lesson_type::Variables {};
    
                    let task = lesson_model::GetLessonType::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::RespLessonType(response),
                    );
                    self.get_lesson_type_task = Some(task);
                }
            }
            Message::RespLessonType(resp) => {
                if resp.is_some() {
                    let lesson_type: Vec<LessonType> = resp.unwrap().robox_lesson_type.iter().map(|item| {
                        match item.lesson_type.clone().as_str() {
                            "TEACHING_CARDS" => LessonType::TeachingCards,
                            "ELECTRONICS_LESSONS" => LessonType::ElectronicsLessons,
                            "EXTRA" => LessonType::Extra,
                            _ => LessonType::Extra,
                        }
                    }).collect();

                    self.robox_lesson_type = lesson_type;
                }
            }
            Message::ShowLessonTypeDropdown => {
                self.show_lesson_type_dropdown = !self.show_lesson_type_dropdown;
            }
            Message::SelectLessonType(select_lesson_type) => {
                self.lesson_type = select_lesson_type
            }
        };
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            // self.link.send_message(Message::FetchLessonById(self.props.lesson_id, self.props.group_id));
            should_render = true;
        }
        should_render
    }

    fn view(&self) -> Html {
        let on_show_del_lesson = self.link.callback(move |_| Message::OnDeleteLessonEntirely);

        let on_show_sidebar = self.link.callback(move |_| Message::ChangeSidebarState);
        let on_dropdown = self.link.callback(|_| Message::ShowLessonTypeDropdown);

        let btn_sidebar_show = if self.props.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick=&on_show_sidebar>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick=&on_show_sidebar>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };
        let lesson_id = self.props.lesson_id;

        // let on_archived_lesson = self.link.callback(move |_| Message::ArchivedLesson(lesson_id));        
        let on_send_to_grade = self.link.callback(move |_| Message::SendLessonToGrade(lesson_id));
        let on_no_send_to_grade = self.link.callback(move |_| Message::NoSendLessonToGrade(lesson_id));

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
                            <img class="img-card-32" src=pic_path alt="" />
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
            let on_del_lesson_entirely = self.link.callback(move |_| Message::DeleteLessonEntirely(lesson_id));

            let naive_default = chrono::NaiveDate::from_ymd_opt(2022, 01, 01).unwrap().and_hms_opt(01, 01, 01).unwrap();
            let naivedatetime = chrono::NaiveDate::from_ymd_opt(2023, 01, 05).unwrap().and_hms_opt(12, 01, 01).unwrap();
            let _timestamp_filter = if lesson.lesson_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive_default).gt(&naivedatetime) {
                true
            } else {
                false
            };

            let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

            let ahothor_id = lesson.clone().lesson_profile.and_then(|data| Some(data.author_id)).unwrap_or(Uuid::default());

            let author_valid = if ahothor_id == user_id.0 {
                true
            } else {
                false
            };

            let selected_lesson_type = match self.lesson_type.is_some() {
                true => match self.lesson_type.clone().unwrap() {
                    LessonType::TeachingCards => "Recurso del Profesor",
                    LessonType::ElectronicsLessons => "Lecciones de Electrónica",
                    LessonType::Extra => "Mis lecciones",
                },
                false => "",
            };

            let class_dropdown = if self.show_lesson_type_dropdown {
                "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
            } else {
                "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
            };
            let class_dropdown_list = if self.show_lesson_type_dropdown {
                "dropdown-menu dropdown-menu-degree show"
            } else {
                "dropdown-menu dropdown-menu-degree"
            };
            let lesson_types = self.robox_lesson_type.iter().map(move |item| {
                let lesson_type = item.clone();
                let l_type = match lesson_type {
                    LessonType::TeachingCards => "Recurso del Profesor",
                    LessonType::ElectronicsLessons => "Lecciones de Electrónica",
                    LessonType::Extra => "Mis lecciones",
                };
        
                let is_checked = Some(lesson_type.clone()) == self.lesson_type;
                let on_select_lesson_type = self.link.callback(move |_| Message::SelectLessonType(Some(lesson_type.clone())));
                html! {
                    <li class="d-flex flex-nowrap">
                        <a class="dropdown-item d-flex flex-nowrap align-items-center mt-1 pe-0" onclick={ on_select_lesson_type }>
                            <input class="bg-checkbox" type="checkbox" checked={ is_checked } />
                            <span class={ if is_checked {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"} } style="white-space: normal !important;">{ l_type }</span>
                        </a>
                    </li>
                }
            }).collect::<Html>();

            let show_lesson_type_dropdown = self
                .props
                .user_profile
                .as_ref()
                .and_then(|user|{
                    if user.user_staff.is_some() {
                        Some(html! {
                            <div class="dropdown me-5">
                                <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" style="width: 275px !important;" onclick=on_dropdown>
                                    <img src="/icons/filter.svg" style="height: 22px;" />
                                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22" style="white-space: normal !important;">{lang::dict(selected_lesson_type)}</span>
                                </button>
                                <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2" style="width: 275px !important;">
                                    { lesson_types }
                                </ul>
                            </div>
                        })
                    } else {
                        Some(html! {})
                    }
                })
                .unwrap_or(html! {});
            
            let maybe_edit_options = match self.edit {
                LessonPageViewEdit::Edit => {
                    let on_done = self
                        .link
                        .callback(move |_| Message::Edit(LessonPageViewEdit::Done));
                    let on_save = self
                        .link
                        .callback(move |_| Message::Edit(LessonPageViewEdit::Save));
                    let on_data = self
                        .link
                        .callback(|data: InputData| Message::Title(data.value));
                    html! {
                        <>
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Lesson Title")} value=self.title.clone() oninput=on_data />
                            // TODO
                            { show_lesson_type_dropdown }
                            <a class="btn button-cancel-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick=on_done>
                                <span class="text-white">
                                    <i class="fas fa-times fas fa-lg"></i>
                                </span>
                            </a>
                            { status_save }
                            <a class="btn button-save-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick=on_save>
                                <span class="text-white">
                                    <i class="fas fa-cloud-upload-alt fas fa-lg"></i>
                                </span>
                            </a>
                        </>
                    }
                }
                _=> {
                    let maybe_lesson_edit = self
                        .props
                        .user_profile
                        .as_ref()
                        .and_then(|item| {
                            let lesson_id = lesson.clone().lesson_profile.and_then(|data| Some(data.lesson_id)).unwrap_or(Uuid::default());
                            let on_edit = self
                                .link
                                .callback( |_| Message::Edit(LessonPageViewEdit::Edit));
                            let on_delete = self
                                .link
                                .callback( move |_| Message::DeleteLessonById(LessonId(lesson_id)));
                            let on_dropdown = self
                                .link
                                .callback( move |_| Message::OnDropdownMenu);
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
                                        <a class=maybe_menu onclick=on_dropdown role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                            <i class="fas fa-ellipsis-v"></i>
                                        </a>
                                        <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
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
                                        <a class=maybe_menu onclick=on_dropdown role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                            <i class="fas fa-ellipsis-v"></i>
                                        </a>
                                        <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
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
                let on_data = self
                    .link
                    .callback(move |data| Message::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                match self.edit {
                    LessonPageViewEdit::Edit => {
                        html! {
                            <div class="mb-6" style="border: 1px solid #C8C1CD; border-radius: 10px; max-height: 90vh; overflow: hidden;">
                                <ckeditor::CKEditor user_profile=self.props.user_profile.clone()
                                    content=self.content.clone()
                                    upload_url=upload_url
                                    on_data=on_data />
                            </div>
                        }
                    }
                    _ => {
                        html! {
                            // <span class="text-dark noir-light is-size-18 lh-22">
                            // </span>
                            
                            <span class="mb-6" style="">
                                {   VNode::VRef(self.node.clone().unwrap()) }
                            </span>
                        }
                    }
                }
            };
            let maybe_messages_right = html! {
                <MessageList on_app_route=self.props.on_app_route.clone() 
                    user_profile=self.props.user_profile.clone() 
                    user_id=None
                    group_category=MessageGroupCategory::Lessons(self.props.group_id, 
                    self.props.lesson_id) />
            };
            let school_id = self.props.school_id;
            let group_id = self.props.group_id;
            let go_back_group = self.link.callback(move |_| Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
            let go_back_grade = html! {
                <>
                    <a onclick=go_back_group>
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

            let class_right_sidebar = if self.props.saved_sidebar_state {
                "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
            } else {
                "d-none"
            };
            let class_sidebar_mobile = if self.props.saved_sidebar_state {
                "offcanvas offcanvas-end show bg-silver d-block d-sm-block d-md-block d-lg-none d-xl-none d-xxl-none"
            } else {
                "offcanvas offcanvas-end"
            };
            let style_sidebar_mobile = if self.props.saved_sidebar_state {
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
                            // <div class="d-flex flex-wrap align-items-center justify-content-between mb-6">
                            <div class="d-flex flex-wrap align-items-center justify-content-between mb-3">
                                {go_back_grade}
                                {btn_sidebar_show}
                            </div>
                            // <div class="d-flex flex-wrap align-items-center justify-content-between mb-4">
                            //     {maybe_edit_options}
                            // </div>
                            // {author_lesson}
                            {maybe_lesson_content}
                        </div>
                    </div>
                    <div class=class_right_sidebar>
                        <div class="d-flex flex-row align-items-center justify-content-between pb-6">
                            <div class="me-5">
                                <SearchLessonGroup on_app_route=self.props.on_app_route.clone()
                                    group_id=self.props.group_id
                                    lesson_id=self.props.lesson_id 
                                    school_id=self.props.school_id />
                            </div>
                            {maybe_user_profile_pic.clone()}
                        </div>
                        <div class="mb-3">
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Notes and comments")}</span>
                        </div>
                        {maybe_messages_right.clone()}
                    </div>
                    <div class=class_sidebar_mobile data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style=style_sidebar_mobile>
                        <div class="offcanvas-header d-flex justify-content-end">
                            <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick=&on_show_sidebar>
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="offcanvas-body pt-0">
                            <div class="d-flex flex-row align-items-center justify-content-between pb-6">
                                <div class="me-5">
                                    <SearchLessonGroup on_app_route=self.props.on_app_route.clone()
                                        group_id=self.props.group_id
                                        lesson_id=self.props.lesson_id 
                                        school_id=self.props.school_id />
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