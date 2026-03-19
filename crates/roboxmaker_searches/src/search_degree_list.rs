use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_main::lang;
use roboxmaker_models::grade_model;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, SchoolId, AppRoute, ClassGroupCategory, LoadResponse, LoadResponseFound};
use yew_router::scope_ext::RouterScopeExt;

pub struct SearchDegreeList {
    graphql_task: Option<GraphQLTask>,
    degree_list_task: Option<RequestTask>,
    search_node: NodeRef,
    grades: Vec<grade_model::grades_by_name::GradesByNameClassGroup>,
    maybe_section_search: bool,
    list_search_state: LoadResponse,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchDegreeListProps {
    // pub on_app_route: Callback<AppRoute>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum SearchDegreeListMessage {
    // AppRoute(AppRoute),
    FetchGradesByName(String),
    GradesSearch(Option<grade_model::grades_by_name::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchDegreeList {
    type Message = SearchDegreeListMessage;
    type Properties = SearchDegreeListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SearchDegreeList { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            degree_list_task: None,
            search_node: NodeRef::default(),
            grades: vec![],
            maybe_section_search: false,
            list_search_state: LoadResponse::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            // SearchDegreeListMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route.clone());
            // }
            SearchDegreeListMessage::FetchGradesByName(search) => {
                let school_id = ctx.props().school_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = grade_model::grades_by_name::Variables {
                        search: format!("%{}%", search), 
                        school_id: school_id.0, 
                    };
                    let task = grade_model::GradesByName::request(
                        graphql_task, 
                        &ctx, 
                        vars, 
                        |response| {
                            SearchDegreeListMessage::GradesSearch(response)
                        }
                    );
                    self.degree_list_task = Some(task);
                }
            }
            SearchDegreeListMessage::GradesSearch(grades) => {
                self.grades = grades.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]);

                if !grades.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadResponse::Load(LoadResponseFound::Found);
                } else {
                    self.list_search_state = LoadResponse::Load(LoadResponseFound::NotFound);
                }
            }
            SearchDegreeListMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchDegreeListMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.maybe_section_search = false;
                self.list_search_state = LoadResponse::Loading;
            }
            SearchDegreeListMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.grades = vec![];
                self.list_search_state = LoadResponse::Loading;
            }
        }
        should_update
    }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_focus = ctx.link().callback(move |_| SearchDegreeListMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| SearchDegreeListMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| SearchDegreeListMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| SearchDegreeListMessage::FetchGradesByName(get_value_from_input_event(search)));
        
        let grades = self
            .grades
            .iter()
            .map(|grade| {
                let group_id = GroupId(grade.group_id);
                let navigator = ctx.link().navigator().unwrap();

                let name = grade.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                let school_id = ctx.props().school_id;
                let category = ClassGroupCategory::Posts;
                let on_menu_school_class_groups = Callback::from(move |_| {
                    navigator.push(&AppRoute::SchoolGroupSection{
                        school_id,
                        group_id,
                        category,
                    })
                });
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_menu_school_class_groups}>
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

        let maybe_message_response = match self.list_search_state {
            LoadResponse::Loading => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Degrees")}</span></span>
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ grades }</div>
                }
            },
            LoadResponse::Load(LoadResponseFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-22 lh-20">{"No se encontró en "}{lang::dict("Degrees")}</span>
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
                <a class="button-search-univeral mt-3 me-5" onclick={&on_hidden_modal}>
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
                                        oninput={on_search.clone()} onfocus={on_focus.clone()} onblur={on_blur.clone()} placeholder={lang::dict("Search")} />
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
