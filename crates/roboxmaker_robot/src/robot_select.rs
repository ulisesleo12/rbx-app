use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew_router::scope_ext::RouterScopeExt;
use yew::{html, Component, Html, Properties};

use roboxmaker_main::lang;
use roboxmaker_models::robot_model;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{RobotId, GroupId, UserId, AppRoute, MyUserProfile};

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
pub enum RobotSelectOption {
    Robot(RobotId),
}

pub struct RobotSelect {
    graphql_task: Option<GraphQLTask>,
    robot_task: Option<RequestTask>,
    robots: Vec<robot_model::robots_by_name::RobotsByNameRobot>,
    search_node: NodeRef,
    show_create: bool,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotSelectProperties {
    pub on_select: Callback<RobotSelectOption>,
    pub allow_create: bool,
    pub group_id: Option<GroupId>,
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub user_id: Option<UserId>,
}

#[derive(Debug)]
pub enum RobotSelectMessage {
    FetchRobotsByRobotName(String),
    Robots(Option<robot_model::robots_by_name::ResponseData>),
    SelectRobot(RobotSelectOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for RobotSelect {
    type Message = RobotSelectMessage;
    type Properties = RobotSelectProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        RobotSelect {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_task: None,
            robots: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_render = true;
        match msg {
            RobotSelectMessage::FetchRobotsByRobotName(search) => {
                should_render = false;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::robots_by_name::Variables {
                        search: format!("%{}%", search)
                    };

                    let task = robot_model::RobotsByName::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                RobotSelectMessage::Robots(response)
                            },
                    );
                    self.robot_task = Some(task);
                }
            }
            RobotSelectMessage::Robots(response) => {
                self.robots = response.clone().and_then(|data| Some(data.robot)).unwrap_or(vec![]);

                if !response.clone().and_then(|data| Some(data.robot)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            RobotSelectMessage::SelectRobot(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.maybe_section_search = false;
                self.robots = vec![];
                ctx.props().on_select.emit(select_option);
            }
            RobotSelectMessage::OnFocus => {
                self.show_create = true;
                self.maybe_section_search = true;
            }
            RobotSelectMessage::OnBlur => {
                self.show_create = false;
                // self.maybe_section_search = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.robots = vec![];
                self.list_search_state = LoadSearch::Static;
            }
            RobotSelectMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.robots = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_focus = ctx.link().callback(move |_| RobotSelectMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| RobotSelectMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| RobotSelectMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| RobotSelectMessage::FetchRobotsByRobotName(get_value_from_input_event(search)));

        let user_id = if let Some(user_id) = ctx.props().user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };
        let group_id = if let Some(group_id) = ctx.props().group_id {
            group_id
        } else {
            GroupId(Uuid::default())
        };
        let robots = self
            .robots
            .iter()
            .map(|robot| {
                let robot_id = RobotId(robot.id);
                let on_select = ctx.link().callback(move |_| {
                    RobotSelectMessage::SelectRobot(RobotSelectOption::Robot(robot_id))
                });
                let name = robot.robot_profile.clone().unwrap().name;

                let navigator = ctx.link().navigator().unwrap();
                let on_robot = Callback::from(move |_| navigator.push(&AppRoute::Robot{robot_id, group_id, user_id}));
                // let on_robot = ctx.link().callback(move |_| RobotSelectMessage::AppRoute(AppRoute::Robot{robot_id, group_id, user_id}));  
                let group_uuid = robot.robot_profile.clone().and_then(|data| data.robot_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Uuid::default());
                let maybe_option = if ctx.props().group_id == Some(GroupId(group_uuid)) {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_robot}>
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
                                    {&name}
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
        let _maybe_option_user = ctx.props().user_profile.as_ref().and_then(|item| {
            if item.user_staff.is_some() || item.user_teacher.is_some() {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Todos los Robots"}</span>
                })
            } else {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Robots del grupo"}</span>
                })
            }
        }).unwrap_or(html! {});
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
                    <div class="d-flex flex-wrap justify-content-center">{ robots }</div>
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
