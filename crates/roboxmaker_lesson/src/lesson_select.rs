use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::{lesson_model};
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_types::types::{GroupId, LessonId, AppRoute, SchoolId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

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
pub enum LessonSelectOption {
    Lesson(LessonId),
}

pub struct LessonSelect {
    graphql_task: Option<GraphQLTask>,
    lesson_task: Option<RequestTask>,
    lessons: Vec<lesson_model::lessons_by_name::LessonsByNameLesson>,
    search_node: NodeRef,
    show_create: bool,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonSelectProperties {
    pub on_select: Callback<LessonSelectOption>,
    pub allow_create: bool,
    #[prop_or(None)]
    pub group_id: Option<GroupId>,
    #[prop_or(None)]
    pub lesson_id: Option<LessonId>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum LessonSelectMessage {
    FetchLessonsByLessonName(String),
    Lessons(Option<lesson_model::lessons_by_name::ResponseData>),
    SelectLesson(LessonSelectOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for LessonSelect {
    type Message = LessonSelectMessage;
    type Properties = LessonSelectProperties;

    fn create(_ctx: &Context<Self>) -> Self {

        LessonSelect {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            lesson_task: None,
            lessons: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            LessonSelectMessage::FetchLessonsByLessonName(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = lesson_model::lessons_by_name::Variables { 
                        search: format!("%{}%", search)
                    };

                    let task = lesson_model::LessonsByName::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            LessonSelectMessage::Lessons(response)
                        },
                    );
                    self.lesson_task = Some(task);
                }
            }
            LessonSelectMessage::Lessons(lessons) => {
                self.lessons = lessons.clone().and_then(|data| Some(data.lesson)).unwrap_or(vec![]);

                if !lessons.clone().and_then(|data| Some(data.lesson)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }

            LessonSelectMessage::SelectLesson(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.maybe_section_search = false;
                self.lessons = vec![];
                ctx.props().on_select.emit(select_option);
            }
            LessonSelectMessage::OnFocus => {
                self.show_create = true;
                self.maybe_section_search = true;
            }
            LessonSelectMessage::OnBlur => {
                self.show_create = false;
                // self.maybe_section_search = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.lessons = vec![];
                self.maybe_section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            LessonSelectMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.lessons = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_focus = ctx.link().callback(move |_| LessonSelectMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| LessonSelectMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| LessonSelectMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| LessonSelectMessage::FetchLessonsByLessonName(get_value_from_input_event(search)));

        let group_id = if let Some(group_id) = ctx.props().group_id {
            group_id
        } else {
            GroupId(Uuid::default())
        };
        let lessons = self
            .lessons
            .iter()
            .map(|lesson| {
                let lesson_id = LessonId(lesson.id);
                let school_id = ctx.props().school_id;

                let on_select = ctx.link().callback(move |_| {
                    LessonSelectMessage::SelectLesson(LessonSelectOption::Lesson(lesson_id))
                });

                let navigator = ctx.link().navigator().unwrap();
                let on_lesson = Callback::from(move |_| navigator.push(&AppRoute::LessonView{school_id, group_id, lesson_id}));

                let title = lesson.lesson_profile.clone().unwrap().title;
                let group_uuid = lesson.lesson_profile.clone().and_then(|data| data.lesson_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Uuid::default());
                let maybe_option = if ctx.props().group_id == Some(GroupId(group_uuid)) {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_lesson}>
                            <span>
                                {lang::dict("View")}
                            </span>
                        </a>
                    }
                } else {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_select}>
                            <span>
                                {lang::dict("Add")}
                            </span>
                        </a>
                    }
                };
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&title}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                {maybe_option}
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
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="pl-2">{lang::dict("Lessons")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ lessons }</div>
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
                <a class="button-search-univeral mt-3" onclick={&on_hidden_modal}>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
                    </span>
                </a>
                <div class={class_search_modal} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_search_scroll} aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class" ref={self.search_node.clone()}
                                        oninput={on_search.clone() }onfocus={on_focus.clone()} onblur={on_blur.clone()} placeholder={lang::dict("Search")} />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick={&on_hidden_modal}>
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
