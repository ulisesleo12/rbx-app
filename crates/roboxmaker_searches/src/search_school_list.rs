use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{SchoolId, AppRoute, LoadResponse, LoadResponseFound};
use yew_router::scope_ext::RouterScopeExt;

pub struct SearchSchoolList {
    graphql_task: Option<GraphQLTask>,
    search_school_task: Option<RequestTask>,
    search_node: NodeRef,
    maybe_section_search: bool,
    school: Vec<school_model::search_school_by_name::SearchSchoolByNameSchool>,
    list_search_state: LoadResponse,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchSchoolListProps {}

#[derive(Debug)]
pub enum SearchSchoolListMessage {
    // AppRoute(AppRoute),
    FetchSchoolByName(String),
    School(Option<school_model::search_school_by_name::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchSchoolList {
    type Message = SearchSchoolListMessage;
    type Properties = SearchSchoolListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SearchSchoolList {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            search_school_task: None,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            school: vec![],
            list_search_state: LoadResponse::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            // SearchSchoolListMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route);
            // }
            SearchSchoolListMessage::FetchSchoolByName(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::search_school_by_name::Variables {
                        search: format!("%{}%", search),
                    };

                    let task = school_model::SearchSchoolByName::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                SearchSchoolListMessage::School(response)
                            },
                    );
                    self.search_school_task = Some(task);
                }
            }
            SearchSchoolListMessage::School(school) => {
                self.school = school.clone().and_then(|data| Some(data.school)).unwrap_or(vec![]);

                if !school.clone().and_then(|data| Some(data.school)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadResponse::Load(LoadResponseFound::Found);
                } else {
                    self.list_search_state = LoadResponse::Load(LoadResponseFound::NotFound);
                }
            }
            SearchSchoolListMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchSchoolListMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.school = vec![];
                self.list_search_state = LoadResponse::Loading;
            }
            SearchSchoolListMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.school = vec![];
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
        let on_focus = ctx.link().callback(move |_| SearchSchoolListMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| SearchSchoolListMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| SearchSchoolListMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| SearchSchoolListMessage::FetchSchoolByName(get_value_from_input_event(search)));

        let search_school_data = self
            .school
            .iter()
            .map(|school| {
                let school_id = SchoolId(school.id);
                let navigator = ctx.link().navigator().unwrap();

                let on_show_school = Callback::from(move |_| navigator.push(&AppRoute::GradesBySchoolId{school_id}));

                let name = school.school_profile.clone().unwrap().name;
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={&on_show_school}>
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
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Schools")}</span></span>
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ search_school_data }</div>
                }
            },
            LoadResponse::Load(LoadResponseFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Schools")}</span>
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