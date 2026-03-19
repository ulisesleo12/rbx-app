use log::*;
use uuid::Uuid;
use yew::web_sys;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use roboxmaker_main::lang;
use roboxmaker_models::robot_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{RobotId, UserId, GroupId, AppRoute, LoadSearch, LoadSearchFound};

pub struct SearchRobotdGroup {
    link: ComponentLink<Self>,
    props: SearchRobotdGroupProperties,
    graphql_task: Option<GraphQLTask>,
    robot_task: Option<RequestTask>,
    robots_by_grade: Option<robot_model::search_by_robot_grade_by_group_id::SearchByRobotGradeByGroupIdGroupByPk>,
    search_node: NodeRef,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchRobotdGroupProperties {
    pub on_app_route: Option<Callback<AppRoute>>,
    pub user_id: Option<UserId>,
    pub group_id: GroupId,
}

#[derive(Debug)]
pub enum SearchRobotdGroupMessage {
    AppRoute(AppRoute),
    FetchRobotsByRobotsGrade(String),
    RobotsByGrade(Option<robot_model::search_by_robot_grade_by_group_id::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchRobotdGroup {
    type Message = SearchRobotdGroupMessage;
    type Properties = SearchRobotdGroupProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SearchRobotdGroup {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_task: None,
            robots_by_grade: None,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            SearchRobotdGroupMessage::AppRoute(route) => {
                if let Some(on_app_route) = &self.props.on_app_route {
                    on_app_route.emit(route)
                }
            }
            SearchRobotdGroupMessage::FetchRobotsByRobotsGrade(search) => {
                self.list_search_state = LoadSearch::Static;

                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::search_by_robot_grade_by_group_id::Variables {
                        search, 
                        group_id: self.props.group_id.0
                    };

                    let task = robot_model::SearchByRobotGradeByGroupId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                SearchRobotdGroupMessage::RobotsByGrade(response)
                            },
                    );
                    self.robot_task = Some(task);
                }
            }
            SearchRobotdGroupMessage::RobotsByGrade(response) => {
                self.robots_by_grade = response.clone().and_then(|data| data.group_by_pk);

                if !response.clone().and_then(|data| data.group_by_pk).clone().and_then(|data| Some(data.robot_groups)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            SearchRobotdGroupMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchRobotdGroupMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.maybe_section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            SearchRobotdGroupMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.robots_by_grade = None;
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_render
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
        let on_focus = self.link.callback(move |_| SearchRobotdGroupMessage::OnFocus);
        let on_blur = self.link.callback(move |_| SearchRobotdGroupMessage::OnBlur);
        let on_hidden_modal = self.link.callback(move |_| SearchRobotdGroupMessage::HiddenModal);
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 0 {
                format!("%{}%", search.value)
            } else {
                search.value
            };
            SearchRobotdGroupMessage::FetchRobotsByRobotsGrade(search)
        });
        let group_id = self.props.group_id;
        let user_id = if let Some(user_id) = self.props.user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };
        let robots_by_grade = self
            .robots_by_grade.clone().and_then(|data| Some(data.robot_groups))
            .unwrap_or(vec![])
            .iter()
            .map(|data| {
                let name = data.robot_profile.clone().and_then(|robot_profile| Some(robot_profile.name)).unwrap_or("".to_string());
                let robot_view = data
                    .robot_profile
                    .iter()
                    .map(|robot_profile | {
                        let robot_id = RobotId(robot_profile.robot_id);
                        let on_robot = self.link.callback(move |_| SearchRobotdGroupMessage::AppRoute(AppRoute::Robot(robot_id, group_id, user_id)));  
                        html! {
                            <a class="btn btn-outline-secondary btn-sm mx-auto" onclick={&on_robot}>
                                <span>
                                    {lang::dict("View")}
                                </span>
                            </a>
                        }
                    })
                    .collect::<Html>();
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                {robot_view}
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
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Robots")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ robots_by_grade }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Robots")}</span>
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