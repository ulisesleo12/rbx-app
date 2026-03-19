use log::*;
use uuid::Uuid;
use chrono::Local;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_models::grade_model::{self, search_by_universal_grade_by_group_id::{self, RoboxLessonTypeEnum}};
use roboxmaker_types::types::{GroupId, RobotId, PostId, ClassesId, SchoolId, UserId, AppRoute, ClassGroupCategory, MeetingsId, LessonId, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub enum SearchPage {
    Posts,
    Classes,
    Lessons,
    Robots,
    Meetings,
    Members,
}

pub struct SearchView {
    graphql_task: Option<GraphQLTask>,
    search_data_task: Option<RequestTask>,
    universal_search: Option<grade_model::search_by_universal_grade_by_group_id::ResponseData>,
    search_node: NodeRef,
    maybe_section_search: bool,
    tab_search_mode: SearchPage,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchProps{
    pub group_id: Option<GroupId>,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum SearchMessage {
    FetchUniversalSearch(String),
    SearchData(Option<grade_model::search_by_universal_grade_by_group_id::ResponseData>),
    OnFocus,
    OnBlur,
    TabSearchPage(SearchPage),
    HiddenModal,
}

impl Component for SearchView {
    type Message = SearchMessage;
    type Properties = SearchProps;

    fn create(ctx: &Context<Self>) -> Self {

        info!("CREATESEARCH {:?}", ctx.props().group_id);
        info!("CREATESEARCH {:?}", ctx.props().school_id);
        info!("CREATESEARCH {:?}", ctx.props().user_profile);

        SearchView {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            search_data_task: None,
            universal_search: None,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            tab_search_mode: SearchPage::Posts,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("update {:?}", msg);
        let should_render = true;
        match msg {
            SearchMessage::FetchUniversalSearch(search) => {

                let scheduled_meetings = Local::now().date_naive();
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(group_id) = ctx.props().group_id {
                        let vars = grade_model::search_by_universal_grade_by_group_id::Variables {
                            search: format!("%{}%", search), 
                            group_id: group_id.0,
                            schedule_time: scheduled_meetings,
                        };
                        let task = grade_model::SearchByUniversalGradeByGroupId::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                SearchMessage::SearchData(response)
                            }
                        );
                        self.search_data_task = Some(task);
                    }
                }
            }
            SearchMessage::SearchData(response) => {
                self.universal_search = response
            }
            SearchMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
            }
            SearchMessage::TabSearchPage(tab) => self.tab_search_mode = tab,
            SearchMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.universal_search = None;
                self.tab_search_mode = SearchPage::Posts;
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let navigator = ctx.link().navigator().unwrap();

        let group_id = if let Some(group_id) = ctx.props().group_id {
            group_id
        } else {
            GroupId(Uuid::default())
        };
        let user_id = UserId(Uuid::default());
        let on_focus = ctx.link().callback(move |_| SearchMessage::OnFocus);
        // let on_blur = ctx.link().callback(move |_| SearchMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| SearchMessage::HiddenModal);

        let on_search = ctx.link().callback(|search| SearchMessage::FetchUniversalSearch(get_value_from_input_event(search)));

        let posts_by_grade = self
            .universal_search.clone()
            .and_then(|data| Some(data.post_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let topic = data.topic.clone();
                let post_id = PostId(data.post_id);
                let school_id = ctx.props().school_id;

                let navigator = navigator.clone();
                let on_post = Callback::from(move |_| navigator.push(&AppRoute::PostView{school_id, group_id, post_id}));

                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&topic}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_post}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();


        let classes_by_grade = self
            .universal_search.clone().and_then(|data| Some(data.classes_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let topic = data.topic.clone();
                let classes_id = ClassesId(data.classes_id);
                let school_id = ctx.props().school_id;

                let navigator = navigator.clone();
                let on_classes = Callback::from(move |_| navigator.push(&AppRoute::Classes{school_id, group_id, classes_id}));

                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&topic}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_classes}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

            
        let lesson_by_grade = self
            .universal_search.clone().and_then(|data| Some(data.lesson_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));
                
                let no_student = ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false);
                
                let title = data.title.clone();
                let lesson_id = LessonId(data.lesson_id);
                let school_id = ctx.props().school_id;

                let author_id = data.author_id;

                let navigator = navigator.clone();
                let on_lesson = Callback::from(move |_| navigator.push(&AppRoute::LessonView{school_id, group_id, lesson_id}));

                let lesson_type = data.lesson_type.clone().unwrap_or(search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::Extra); 

                let content = data.lesson_content.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string());

                if no_student {
                    if lesson_type == search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::TeachingCards {
                        if author_id == user_id.0 {
                            html! {
                                <div class="m-4">
                                    <div class="card card-search-u vh-15">
                                        <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                                {&title}
                                            </span>
                                        </div>
                                        <div class="card-body border-top d-flex px-5 py-2">
                                            <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_lesson}>
                                                <span>
                                                    {lang::dict("View")}
                                                </span>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="m-4">
                                    <div class="card card-search-u vh-15">
                                        <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                                {&title}
                                            </span>
                                        </div>
                                        <div class="card-body border-top d-flex px-5 py-2">
                                            <ContentLesson content={content.clone()} title={title.clone()} />
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    } else if lesson_type == search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::ElectronicsLessons {
                        html! {
                            <div class="m-4">
                                <div class="card card-search-u vh-15">
                                    <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                            {&title}
                                        </span>
                                    </div>
                                    <div class="card-body border-top d-flex px-5 py-2">
                                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_lesson}>
                                            <span>
                                                {lang::dict("View")}
                                            </span>
                                        </a>
                                    </div>
                                </div>
                            </div>
                        }
                    } else if lesson_type == search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::Extra {
                        if author_id == user_id.0 {
                            html! {
                                <div class="m-4">
                                    <div class="card card-search-u vh-15">
                                        <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                                {&title}
                                            </span>
                                        </div>
                                        <div class="card-body border-top d-flex px-5 py-2">
                                            <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_lesson}>
                                                <span>
                                                    {lang::dict("View")}
                                                </span>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                } else {
                    html! {
                        <div class="m-4">
                            <div class="card card-search-u vh-15">
                                <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                        {&title}
                                    </span>
                                </div>
                                <div class="card-body border-top d-flex px-5 py-2">
                                    <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_lesson}>
                                        <span>
                                            {lang::dict("View")}
                                        </span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    }
                }
            })
            .collect::<Html>();

        let robots_by_grade = self
            .universal_search.clone().and_then(|data| Some(data.robot_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let name = data.name.clone();
                let robot_id = RobotId(data.robot_id);

                let navigator = navigator.clone();
                let on_robot = Callback::from(move |_| navigator.push(&AppRoute::Robot{robot_id, group_id, user_id}));
                
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_robot}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        let meetings_by_grade = self
            .universal_search.clone().and_then(|data| Some(data.meetings_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let title = data.title.clone();
                let meetings_id = MeetingsId(data.meet_id);

                let navigator = navigator.clone();
                let on_meet = Callback::from(move |_| navigator.push(&AppRoute::Meet{group_id, meetings_id}));

                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&title}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_meet}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        let members_by_grade = self
            .universal_search.clone().and_then(|data| Some(data.user_profile))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let full_name = data.full_name.clone();
                let school_id = ctx.props().school_id;
                let category = ClassGroupCategory::Members;


                let navigator = navigator.clone();
                let on_list_users = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));

                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&full_name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_list_users}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        let on_posts = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Posts));
        let on_classes = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Classes));
        let on_lessons = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Lessons));
        let on_robots = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Robots));
        let on_meetings = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Meetings));
        let on_members = ctx.link().callback(|_| SearchMessage::TabSearchPage(SearchPage::Members));
        
        let page_mode = match self.tab_search_mode {
            SearchPage::Posts => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{posts_by_grade}</div>
                }
            },
            SearchPage::Classes => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{classes_by_grade}</div>
                }
            },
            SearchPage::Lessons => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{lesson_by_grade}</div>
                }
            },
            SearchPage::Robots => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{robots_by_grade}</div>
                }
            },
            SearchPage::Meetings => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{meetings_by_grade}</div>
                }
            },
            SearchPage::Members => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{members_by_grade}</div>
                }
            },
        };
        let maybe_response_search = self.universal_search.iter().map(|universal_search| {
            let maybe_response = match self.tab_search_mode {
                SearchPage::Posts => {
                    if !universal_search.post_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Posts")}</span>
                            </div>
                        }
                    }
                },
                SearchPage::Classes => {
                    if !universal_search.classes_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Classes")}</span>
                            </div>
                        }
                    }
                },
                SearchPage::Lessons => {
                    if !universal_search.lesson_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Lessons")}</span>
                            </div>
                        }
                    }
                },
                SearchPage::Robots => {
                    if !universal_search.robot_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Robots")}</span>
                            </div>
                        }
                    }
                },
                SearchPage::Meetings => {
                    if !universal_search.meetings_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Meetings")}</span>
                            </div>
                        }
                    }
                },
                SearchPage::Members => {
                    if !universal_search.user_profile.is_empty() {
                        html! {}
                    } else {
                        html! {
                            <div class="d-flex justify-content-center">
                                <span class="text-danger is-size-20 lh-20">{"No se encontraron "}{lang::dict("Members")}</span>
                            </div>
                        }
                    }
                },
            };
            html! {
                {maybe_response}
            }
        }).collect::<Html>();
        let class_tab_search = |flag: bool | match flag {
            true => "nav-link active is-active-tab",
            false => "nav-link is-no-active-tab",
        };
        let maybe_message_search = if self.universal_search.iter().cloned().len() == 0 {
            html! {
                <div class="text-center">
                    <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the universal search engine")}</span>
                </div>
            }
        } else {
            let maybe_tabs = self.universal_search.iter().zip(ctx.props().user_profile.clone()).map(|(search, user)| {
                let members_search =  if user.user_staff.is_some() || user.user_teacher.is_some() {
                    html! {
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Members)} onclick={on_members.clone()}>
                                <img src="/icons/user-class-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Members")}<span class="ms-1">{&search.user_profile.len()}</span></span>
                            </a>
                        </li>
                    }
                } else {html! {}};

                let user_id = user.user_id.0;
                // // let author_id = search.lesson_profile.iter().filter(||);

                let lesson_teaching_cards = search
                    .lesson_profile
                    .iter()
                    .filter(|item| 
                        item.lesson_type.clone()
                        .unwrap_or(search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::Extra) == RoboxLessonTypeEnum::TeachingCards || 
                        item.lesson_type.clone()
                        .unwrap_or(search_by_universal_grade_by_group_id::RoboxLessonTypeEnum::Extra) == RoboxLessonTypeEnum::ElectronicsLessons || 
                        item.author_id == user_id
                    ).count();

                let maybe_lessons = if user.user_staff.is_some() || user.user_teacher.is_some() {
                    html! {
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Lessons)} onclick={&on_lessons}>
                                <img src="/icons/folders-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Lessons")}<span class="ms-1">{&lesson_teaching_cards}</span></span>
                            </a>
                        </li>
                    }
                } else {
                    html! {
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Lessons)} onclick={&on_lessons}>
                                <img src="/icons/folders-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Lessons")}<span class="ms-1">{&search.lesson_profile.len()}</span></span>
                            </a>
                        </li>
                    }
                };

                html! {
                    <ul class="nav nav-tabs justify-content-center mb-5">
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Posts)} onclick={on_posts.clone()}>
                                <img src="/icons/envelope-open-text.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Posts")}<span class="ms-1">{&search.post_profile.len()}</span></span>
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Classes)} onclick={on_classes.clone()}>
                                <img src="/icons/folders-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Classes")}<span class="ms-1">{&search.classes_profile.len()}</span></span>
                            </a>
                        </li>
                        {maybe_lessons}
                        // <li class="nav-item">
                        //     <a class={class_tab_search(self.tab_search_mode==SearchPage::Lessons)} onclick={&on_lessons}>
                        //         <img src="/icons/folders-2.svg" style="height: 22px;" />
                        //         <span class="ms-1">{lang::dict("Lessons")}<span class="ms-1">{&search.lesson_profile.len()}</span></span>
                        //     </a>
                        // </li>
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Robots)} onclick={on_robots.clone()}>
                                <img src="/icons/robot-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Robots")}<span class="ms-1">{&search.robot_profile.len()}</span></span>
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class={class_tab_search(self.tab_search_mode==SearchPage::Meetings)} onclick={on_meetings.clone()}>
                                <img src="/icons/video-2.svg" style="height: 22px;" />
                                <span class="ms-1">{lang::dict("Meetings")}<span class="ms-1">{&search.meetings_profile.len()}</span></span>
                            </a>
                        </li>
                        {members_search}
                    </ul>
                }
            }).collect::<Html>();
            html! {
                {maybe_tabs}
            }
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
                <div class={class_search_modal} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_search_scroll} aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class"
                                        oninput={on_search} onfocus={on_focus} placeholder={lang::dict("Search")} />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick={&on_hidden_modal}>
                                    <span class="text-white">
                                        <i class="fas fa-times"></i>
                                    </span>
                                </a>
                            </div>
                            <div class="modal-body vh-100">
                                {maybe_message_search}
                                {maybe_response_search}
                                {page_mode}
                            </div>
                        </div>
                    </div>
                </div>
                <a class="button-search-univeral mt-3" onclick={&on_hidden_modal}>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
                    </span>
                </a>
            </>
        }
    }
}


pub struct ContentLesson {
    link_download: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ContentLessonProps {
    pub content: String,
    pub title: String,
}

#[derive(Debug)]
pub enum ContentLessonMessage {}

impl Component for ContentLesson {
    type Message = ContentLessonMessage;
    type Properties = ContentLessonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let start = "<a href=\'";
        let end = "\'>".to_owned() + &ctx.props().title.clone() + "</a>";
        let maybe_content = ctx.props().content.clone();
        let content = maybe_content.trim_start_matches(start);
        let link_download = content.trim_end_matches(&end);
        ContentLesson {
            link_download: link_download.to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        match msg { }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;
        
        let start = "<a href=\'";
        let end = "\'>".to_owned() + &ctx.props().content.clone() + "</a>";
        let maybe_content = ctx.props().content.clone();
        let content = maybe_content.trim_start_matches(start);
        let link_download = content.trim_end_matches(&end);
        self.link_download = link_download.to_string();

        if ctx.props() != old_props {
            should_render = true;
        } 

        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <a href={self.link_download.clone()} target="_blank" class="btn btn-outline-secondary btn-sm mx-auto">
                    <span>
                        {lang::dict("View")}
                    </span>
                </a>
                // <span>{self.link_download.clone()}</span>
            </>
        }
    }
}