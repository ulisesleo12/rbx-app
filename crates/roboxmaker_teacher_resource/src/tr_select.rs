use log::*;
use uuid::Uuid;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::teacher_resource;
use roboxmaker_types::types::{GroupId, ResourceId, AppRoute, SchoolId};
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
pub enum SelectResourceOption {
    Resource(ResourceId),
}

pub struct SelectResource {
    link: ComponentLink<Self>,
    props: SelectResourceProperties,
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    resources: Vec<teacher_resource::resources_by_name::ResourcesByNameTeacherResources>,
    search_node: NodeRef,
    show_create: bool,
    section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SelectResourceProperties {
    pub on_select: Callback<SelectResourceOption>,
    pub allow_create: bool,
    pub on_app_route: Callback<AppRoute>,
    pub group_id: Option<GroupId>,
    pub resource_id: Option<ResourceId>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchResourcesByName(String),
    Resources(Option<teacher_resource::resources_by_name::ResponseData>),
    SelectResource(SelectResourceOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SelectResource {
    type Message = Message;
    type Properties = SelectResourceProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SelectResource {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            resources: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            Message::FetchResourcesByName(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::resources_by_name::Variables { 
                        search
                    };

                    let task = teacher_resource::ResourcesByName::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::Resources(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            Message::Resources(resources) => {
                self.resources = resources.clone().and_then(|data| Some(data.teacher_resources)).unwrap_or(vec![]);

                if !resources.clone().and_then(|data| Some(data.teacher_resources)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }

            Message::SelectResource(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.section_search = false;
                self.resources = vec![];
                self.props.on_select.emit(select_option);
            }
            Message::OnFocus => {
                self.show_create = true;
                self.section_search = true;
            }
            Message::OnBlur => {
                self.show_create = false;
                // self.section_search = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.resources = vec![];
                self.section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            Message::HiddenModal => {
                self.section_search = !self.section_search;
                self.resources = vec![];
                self.list_search_state = LoadSearch::Static;
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
        let on_focus = self.link.callback(move |_| Message::OnFocus);
        let on_blur = self.link.callback(move |_| Message::OnBlur);
        let on_hidden_modal = self.link.callback(move |_| Message::HiddenModal);
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 0 {
                format!("%{}%", search.value)
            } else {
                search.value
            };
            Message::FetchResourcesByName(search)
        });

        let group_id = if let Some(group_id) = self.props.group_id {
            group_id
        } else {
            GroupId(Uuid::default())
        };
        let resources = self
            .resources
            .iter()
            .map(|resource| {
                let resource_id = ResourceId(resource.id);
                let school_id = self.props.school_id;

                let on_select = self.link.callback(move |_| {
                    Message::SelectResource(SelectResourceOption::Resource(resource_id))
                });

                let on_resource = self.link.callback(move |_| Message::AppRoute(AppRoute::Resource(school_id, group_id, resource_id)));
                let title = resource.teacher_resource_profile.clone().unwrap().title;
                let group_uuid = resource.teacher_resource_profile.clone().and_then(|data| data.teacher_resource_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Uuid::default());
                let maybe_option = if self.props.group_id == Some(GroupId(group_uuid)) {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown=on_resource>
                            <span>
                                {lang::dict("View")}
                            </span>
                        </a>
                    }
                } else {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown=on_select>
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
        let message_response = match self.list_search_state {
            LoadSearch::Static => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="pl-2">{lang::dict("resources")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ resources }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("resources")}</span>
                    </div>
                }
            },
        };
        let class_search_modal = if self.section_search {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.section_search {
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
                                { message_response }
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
