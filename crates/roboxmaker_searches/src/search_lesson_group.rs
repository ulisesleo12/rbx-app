use log::*;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::lesson_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, LoadSearch, LoadSearchFound, SchoolId};

pub struct SearchLessonGroup {
    link: ComponentLink<Self>,
    props: SearchLessonGroupProperties,
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    lessons_by_grade: Option<lesson_model::search_by_lesson_grade_by_group_id::SearchByLessonGradeByGroupIdGroupByPk>,
    search_node: NodeRef,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchLessonGroupProperties {
    pub on_app_route: Callback<AppRoute>,
    pub lesson_id: LessonId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum SearchLessonGroupMessage {
    AppRoute(AppRoute),
    FetchLessonsByLessonsGrade(String),
    LessonsByGrade(Option<lesson_model::search_by_lesson_grade_by_group_id::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchLessonGroup {
    type Message = SearchLessonGroupMessage;
    type Properties = SearchLessonGroupProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SearchLessonGroup {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            lessons_by_grade: None,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            SearchLessonGroupMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            SearchLessonGroupMessage::FetchLessonsByLessonsGrade(search) => {
                should_update = false;
                let group_id = self.props.group_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::search_by_lesson_grade_by_group_id::Variables { 
                        search, 
                        group_id: group_id.0 
                    };

                    let task = lesson_model::SearchByLessonGradeByGroupId::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            SearchLessonGroupMessage::LessonsByGrade(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            SearchLessonGroupMessage::LessonsByGrade(response) => {
                self.lessons_by_grade = response.clone().and_then(|data| data.group_by_pk);

                if !response.clone().and_then(|data| data.group_by_pk).clone().and_then(|group_by_pk| Some(group_by_pk.lesson_groups)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            SearchLessonGroupMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchLessonGroupMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.maybe_section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            SearchLessonGroupMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.lessons_by_grade = None;
                self.list_search_state = LoadSearch::Static;
            }
        };
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
        let on_focus = self.link.callback(move |_| SearchLessonGroupMessage::OnFocus);
        let on_blur = self.link.callback(move |_| SearchLessonGroupMessage::OnBlur);
        let on_hidden_modal = self.link.callback(move |_| SearchLessonGroupMessage::HiddenModal);
        let group_id = self.props.group_id;
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 0 {
                format!("%{}%", search.value)
            } else {
                search.value
            };
            SearchLessonGroupMessage::FetchLessonsByLessonsGrade(search)
        });

        let lessons_by_grade = self
            .lessons_by_grade.clone().and_then(|data| Some(data.lesson_groups))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let title = data.lesson_profile.clone().and_then(|data| Some(data.title)).unwrap_or("".to_string());
                let lesson_view = data
                    .lesson_profile
                    .iter()
                    .map(|lesson_profile | {
                        let lesson_id = LessonId(lesson_profile.lesson_id);
                        let school_id = self.props.school_id;
                        let on_lessons_view = self.link.callback(move |_| SearchLessonGroupMessage::AppRoute(AppRoute::LessonView(school_id, group_id, lesson_id)));  
                        let on_lessons_edit = self.link.callback(move |_| SearchLessonGroupMessage::AppRoute(AppRoute::Lesson(school_id, group_id, lesson_id)));  
                        html! {
                            <>
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onclick={&on_lessons_edit}>
                                    <span>
                                        {lang::dict("Edit")}
                                    </span>
                                </a>
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onclick={&on_lessons_view}>
                                    <span>
                                        {lang::dict("View")}
                                    </span>
                                </a>
                            </>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&title}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex flex-wrap px-5 py-2">
                                {lesson_view}
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
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Lessons")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ lessons_by_grade }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Lessons")}</span>
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
                <div class=class_search_modal id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style=class_search_scroll aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class" ref=self.search_node.clone()
                                        oninput=on_search.clone() onfocus=on_focus.clone() onblur=on_blur.clone() placeholder=lang::dict("Search") />
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