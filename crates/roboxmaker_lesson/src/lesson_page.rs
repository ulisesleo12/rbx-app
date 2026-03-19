use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use web_sys::{Node, window};
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_graphql::Subscribe;
use roboxmaker_types::types::UserId;
use roboxmaker_main::{lang, config};
use roboxmaker_models::lesson_model;
use roboxmaker_graphql::SubscriptionTask;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_searches::search_lesson_group::SearchLessonGroup;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, SchoolId, ClassGroupCategory, MyUserProfile};
use yew_router::scope_ext::RouterScopeExt;


#[derive(Debug, Clone, PartialEq)]
pub enum LessonMode {
    Edit,
    Preview
}
pub struct LessonPage {
    graphql_task: Option<GraphQLTask>,
    load_task: Option<SubscriptionTask>,
    save_task: Option<RequestTask>,
    lesson: Option<lesson_model::lesson_by_id::LessonByIdLessonGroupByPk>,
    node: Option<Node>,
    title: String,
    content: String,
    save_status: bool,
    tab_page_mode: LessonMode,
    class_name: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonPageProperties {
    // pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum LessonPageMessage {
    // AppRoute(AppRoute),
    FetchLessonById(LessonId, GroupId),
    Lesson(Option<lesson_model::lesson_by_id::ResponseData>),
    SaveLesson,
    Content(String),
    Title(String),
    Saved(Option<lesson_model::lesson_by_id_update::ResponseData>),
    Back,
    TabPageMode(LessonMode),
}

impl Component for LessonPage {
    type Message = LessonPageMessage;
    type Properties = LessonPageProperties;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(LessonPageMessage::FetchLessonById(ctx.props().lesson_id, ctx.props().group_id));

        roboxmaker_utils::functions::school_state();

        LessonPage {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            load_task: None,
            save_task: None,
            lesson: None,
            node: None,
            title: String::from(""),
            content: String::from(""),
            save_status: true,
            tab_page_mode: LessonMode::Edit,
            class_name: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            // LessonPageMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route);
            // }
            LessonPageMessage::FetchLessonById(lesson_id, group_id) => {
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
                            LessonPageMessage::Lesson(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            LessonPageMessage::Lesson(response) => {
                self.lesson = response.clone().and_then(|data| data.lesson_group_by_pk);
                if let Some(lesson) = &self.lesson {
                    self.class_name = lesson.clone().class_profile.and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or("".to_string());

                    if let Some(lesson_profile) = &lesson.lesson_profile {
                        self.title = lesson_profile.title.clone();
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

                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = ctx.props().school_id;
                let group_id = ctx.props().group_id;
                let category = ClassGroupCategory::Lessons;
                
                if response.clone().and_then(|data| data.lesson_group_by_pk).is_none() {
                    if ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        let navigator = ctx.link().navigator().unwrap();
                        navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category});
                    } else {
                        let navigator = ctx.link().navigator().unwrap();
                        
                        navigator.push(&AppRoute::GroupSectionStudent{school_id, user_id, category});
                    }
                }
            }
            LessonPageMessage::Title(title) => {
                self.title = title;
                self.save_status = false;
                should_update = true;
            }
            LessonPageMessage::Content(content) => {
                self.content = content;
                self.save_status = false;
                should_update = true;
            }
            LessonPageMessage::SaveLesson => {
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
                            LessonPageMessage::Saved(response)
                        },
                    );
                    self.save_task = Some(task);
                }
                ctx.link().send_message(LessonPageMessage::Back);
            }
            LessonPageMessage::Saved(response) => {
                if response.clone().and_then(|data| data.update_lesson_profile_by_pk).is_some() &&
                    response.clone().and_then(|data| data.update_lesson_content_by_pk).is_some() {
                    self.save_status = true;
                }
            }
            LessonPageMessage::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            LessonPageMessage::TabPageMode(tab) => {
                self.tab_page_mode = tab;
                if self.tab_page_mode == LessonMode::Preview {
                    ctx.link().send_message(LessonPageMessage::SaveLesson);
                }
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
                                <img class="img-card-32" src={pic_path} />
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
                let on_data = ctx.link().callback(|data: InputEvent| LessonPageMessage::Title(get_value_from_input_event(data)));

                html! {
                    <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Lesson Title")} value={self.title.clone()} oninput={on_data} />
                }
            };

            let maybe_lesson_content_edit = {
                let on_data = ctx
                    .link()
                    .callback(move |data| LessonPageMessage::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                html! {
                    <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone() }
                        content={self.content.clone()}
                        upload_url={upload_url} 
                        on_data={on_data} />
                }
            };
            let maybe_lesson_content_preview = html! {
                VNode::VRef(self.node.clone().unwrap())
            };
            let on_edit = ctx.link().callback(|_| LessonPageMessage::TabPageMode(LessonMode::Edit));
            let on_preview = ctx.link().callback(|_| LessonPageMessage::TabPageMode(LessonMode::Preview));
            let school_id = ctx.props().school_id;
            let group_id = ctx.props().group_id;
            let category = ClassGroupCategory::Lessons;
            let navigator = ctx.link().navigator().unwrap();

            let go_back_group = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            let go_back_grade = html! {
                <>
                    <a onclick={go_back_group} class="mb-2">
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
                        <a class={tab_class(self.tab_page_mode==LessonMode::Edit)} onclick={on_edit.clone()}>{lang::dict("Edit")}</a>
                    </li>
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==LessonMode::Preview)} onclick={on_preview.clone()}>{lang::dict("Preview")}</a>
                    </li>
                </ul>
            };
            let maybe_user_profile_pic = ctx
                .props()
                .user_profile
                .as_ref()
                .and_then(|item| Some(item.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
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
            let status_save = if self.save_status {
                html! {
                    <span class="text-success mx-4">{lang::dict("Saved")}</span>
                }
            } else {
                html! {
                    <span class="text-danger mx-4">{lang::dict("Unsaved")}</span>
                }
            };
            let maybe_save_lesson = ctx
                .props()
                .user_profile
                .as_ref()
                .zip(
                    self.lesson
                        .as_ref()
                        .and_then(|lesson| lesson.lesson_profile.as_ref()),
                )
                .and_then(|(item, lesson_profile)| {
                    let on_save = ctx.link().callback( |_| LessonPageMessage::SaveLesson);
                    if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_id.0 == lesson_profile.author_id {
                        Some(html! {
                            <>
                                {status_save}
                                <a class="button button-saved-lesson bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick={on_save}>
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
                                <SearchLessonGroup group_id={ctx.props().group_id}
                                    lesson_id={ctx.props().lesson_id} 
                                    school_id={ctx.props().school_id} />
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