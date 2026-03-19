use log::*;
use roboxmaker_graphql::Subscribe;
use roboxmaker_graphql::SubscriptionTask;
use roboxmaker_types::types::UserId;
use uuid::Uuid;
use yew::web_sys::Node;
use yew::web_sys::window;
use yew::virtual_dom::VNode;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::{school_model, lesson_model};
use roboxmaker_searches::search_lesson_group::SearchLessonGroup;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, SchoolId, ClassGroupCategory, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub enum LessonType {
    TeachingCards,
    ElectronicsLessons,
    Extra,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LessonMode {
    Edit,
    Preview
}
pub struct LessonPage {
    link: ComponentLink<Self>,
    props: LessonPageProperties,
    graphql_task: Option<GraphQLTask>,
    load_task: Option<SubscriptionTask>,
    save_task: Option<RequestTask>,
    get_lesson_type_task: Option<RequestTask>,
    lesson: Option<lesson_model::lesson_by_id::LessonByIdLessonGroupByPk>,
    node: Option<Node>,
    title: String,
    content: String,
    save_status: bool,
    tab_page_mode: LessonMode,
    class_name: String,
    robox_lesson_type: Vec<LessonType>,
    show_lesson_type_dropdown: bool,
    lesson_type: Option<LessonType>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonPageProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchLessonById(LessonId, GroupId),
    Lesson(Option<lesson_model::lesson_by_id::ResponseData>),
    SaveLesson,
    Content(String),
    Title(String),
    Saved(Option<lesson_model::lesson_by_id_update::ResponseData>),
    Back,
    TabPageMode(LessonMode),

    GetLessonType,
    RespLessonType(Option<lesson_model::get_lesson_type::ResponseData>),
    ShowLessonTypeDropdown,
    SelectLessonType(Option<LessonType>),
}

impl Component for LessonPage {
    type Message = Message;
    type Properties = LessonPageProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::GetLessonType);
        link.send_message(Message::FetchLessonById(props.lesson_id, props.group_id));
        LessonPage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            load_task: None,
            save_task: None,
            get_lesson_type_task: None,
            lesson: None,
            node: None,
            title: String::from(""),
            content: String::from(""),
            save_status: true,
            tab_page_mode: LessonMode::Edit,
            class_name: String::from(""),
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

                        if let Some(lesson_type) = &lesson_profile.lesson_type {
                            let lesson_type = match lesson_type {
                                lesson_model::lesson_by_id::RoboxLessonTypeEnum::ElectronicsLessons => LessonType::ElectronicsLessons,
                                lesson_model::lesson_by_id::RoboxLessonTypeEnum::Extra => LessonType::Extra,
                                lesson_model::lesson_by_id::RoboxLessonTypeEnum::TeachingCards => LessonType::TeachingCards,
                                lesson_model::lesson_by_id::RoboxLessonTypeEnum::Other(_) => LessonType::Extra,
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
            Message::SaveLesson => {
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
                self.link.send_message(Message::Back);
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
            Message::TabPageMode(tab) => {
                self.tab_page_mode = tab;
                if self.tab_page_mode == LessonMode::Preview {
                    // self.link.send_message(Message::SaveLesson);
                    let node = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.create_element("div").ok())
                            .and_then(|div| {
                                div.set_class_name("ck-content");
                                div.set_inner_html(&self.content);
                                Some(Node::from(div))
                            });
                    self.node = node;
                }
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
        let on_dropdown = self.link.callback(|_| Message::ShowLessonTypeDropdown);

        let author_lesson = self
            .lesson
            .as_ref()
            .and_then(|lesson | lesson.lesson_profile.as_ref())
            .and_then(|lesson_profile| {
                let author_profile = lesson_profile.author_profile.as_ref().unwrap();
                let pic_path = author_profile.pic_path.clone().unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_owned());

                Some(html! {
                    <div class="d-flex flex-wrap align-items-center justify-content-between pb-6">
                        <div class="d-flex align-items-center">
                                <img class="img-card-32" src=pic_path />
                            <span class="text-dark noir-light is-size-18 lh-22 ps-2">{&author_profile.full_name}</span>
                        </div>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <span class="icon">
                                <i class="far fa-clock"></i>
                            </span>
                            <span class="ps-2">{&lesson_profile.timestamp.format("%a %b %e %T %Y").to_string()}</span>
                        </span>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <span class="icon">
                                <i class="fas fa-graduation-cap"></i>
                            </span>
                            <span class="ps-2">{self.class_name.clone()}</span>
                        </span>
                    </div>
                })
            })
            .unwrap_or(html! {});
        if let Some(_lesson) = &self.lesson {
            let maybe_lesson_title = {
                let on_data = self
                    .link
                    .callback(|data: InputData| Message::Title(data.value));
                html! {
                    <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Lesson Title")} value=self.title.clone() oninput=on_data />
                }
            };
            let maybe_lesson_content_edit = {
                let on_data = self
                    .link
                    .callback(move |data| Message::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                html! {
                    <ckeditor::CKEditor user_profile=self.props.user_profile.clone() 
                        content=self.content.clone()
                        upload_url=upload_url 
                        on_data=on_data />
                }
            };
            let maybe_lesson_content_preview = html! {
                VNode::VRef(self.node.clone().unwrap())
            };
            let on_edit = self.link.callback(|_| Message::TabPageMode(LessonMode::Edit));
            let on_preview = self.link.callback(|_| Message::TabPageMode(LessonMode::Preview));
            let school_id = self.props.school_id;
            let group_id = self.props.group_id;
            let go_back_group = self.link.callback(move |_| Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
            let go_back_grade = html! {
                <>
                    <a onclick=go_back_group class="mb-2">
                        <span class="text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center">
                            <i class="fas fa-arrow-left"></i>
                            <span class="mx-2">{lang::dict("To Lessons")}</span>
                            {self.class_name.clone()}
                        </span>
                    </a>
                </>
            };
            let tab_class = |flag: bool | match flag {
                true => "nav-link active is-active-tab",
                false => "nav-link is-no-active-tab",
            };
            let maybe_tabs = html! {
                <ul class="nav nav-tabs mb-5">
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==LessonMode::Edit)} onclick=on_edit.clone()>{lang::dict("Edit")}</a>
                    </li>
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==LessonMode::Preview)} onclick=on_preview.clone()>{lang::dict("Preview")}</a>
                    </li>
                </ul>
            };
            let maybe_user_profile_pic = self
                .props
                .user_profile
                .as_ref()
                .and_then(|item| Some(item.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                    })
                })
                .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
                });
            let page_mode = match self.tab_page_mode {
                LessonMode::Edit => {
                    html! {
                        <div style="border: 1px solid #C8C1CD; border-radius: 10px;">
                            {maybe_lesson_content_edit}
                        </div>
                    }
                }
                LessonMode::Preview => {
                    html! {
                        <>
                            <h1 class="text-primary-blue-dark noir-bold is-size-32 lh-38 text-uppercase pb-3">{&self.title}</h1>
                            {author_lesson}
                            <div class="text-dark noir-light is-size-18 lh-22">
                                {maybe_lesson_content_preview}
                            </div>
                        </>
                    }
                }
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

            let status_save = if self.save_status {
                html! {
                    <span class="text-success mx-4">{lang::dict("Saved")}</span>
                }
            } else {
                html! {
                    <span class="text-danger mx-4">{lang::dict("Unsaved")}</span>
                }
            };
            let maybe_save_lesson = self
                .props
                .user_profile
                .as_ref()
                .zip(
                    self.lesson
                        .as_ref()
                        .and_then(|lesson| lesson.lesson_profile.as_ref()),
                )
                .and_then(|(item, lesson_profile)| {
                    let on_save = self.link.callback( |_| Message::SaveLesson);
                    if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_id.0 == lesson_profile.author_id {
                        Some(html! {
                            <>
                                { show_lesson_type_dropdown }
                                
                                { status_save }
                                <a class="button button-saved-lesson bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick=on_save>
                                    <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save")}</span>
                                </a>
                            </>
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(html! {});
            html! {
                <>
                    <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
                        <div class="d-flex flex-wrap align-items-center justify-content-between">
                            {go_back_grade}
                            <div class="d-flex flex-row align-items-center">
                                <div class="mx-5">
                                <SearchLessonGroup on_app_route=self.props.on_app_route.clone()
                                    group_id=self.props.group_id
                                    lesson_id=self.props.lesson_id 
                                    school_id=self.props.school_id />
                                </div>
                                {maybe_user_profile_pic}
                            </div>
                        </div>
                        <h1 class="text-primary-blue-light noir-bold is-size-24 lh-30 mb-0">{lang::dict("New Lesson")}</h1>
                        <div class="d-flex flex-wrap align-items-center justify-content-between pt-4 pb-6">
                            {maybe_lesson_title}
                            {maybe_save_lesson}
                        </div>
                        {maybe_tabs}
                        {page_mode}
                    </div>
                </>
            }
        } else {
            html! {
                <div class="progress w-100">
                    <div class="progress-bar" role="progressbar" style="width: 100%;" aria-valuenow="100" aria-valuemin="0" aria-valuemax="100">{"100%"}</div>
                </div>
            }
        }
    }
}