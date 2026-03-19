use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::robots_card::RobotsCard;
use crate::robot_select::RobotSelect;
use crate::robot_select::RobotSelectOption;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use roboxmaker_main::lang;
use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, robot_model};
use roboxmaker_searches::search_robots_group::SearchRobotdGroup;
use roboxmaker_models::robot_model::{robots_list_by_group, robot_group_add};
use roboxmaker_types::types::{RobotId, GroupId, UserId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};

#[derive(Debug, Clone, PartialEq)]
pub struct RobotProfile {
    pub name: String,
    pub path: String,
    pub timestamp: String,
    pub robot_id: RobotId,
    pub enabled: bool,
    pub robot_type: robots_list_by_group::RoboxRobotTypeEnum,
}

pub struct RobotsList {
    link: ComponentLink<Self>,
    props: RobotsListProperties,
    graphql_task: Option<GraphQLTask>,
    robot_sub: Option<SubscriptionTask>,
    robot_delete_task: Option<RequestTask>,
    robot_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: RobotFilter,
    robot_list: Vec<RobotProfile>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotFilter {
    Alls,
    Enabled,
    Disabled,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotsListProperties {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_id: Option<UserId>,
    pub on_app_route: Callback<AppRoute>,
    pub class_name: String,
}

#[derive(Debug)]
pub enum RobotsListMessage {
    AppRoute(AppRoute),
    FetchRobotsByGroupId,
    Robots(Option<robot_model::robots_list_by_group::ResponseData>),
    AddRobot(RobotId),
    RemoveRobot(RobotId),
    RobotAdded(Option<RobotId>),
    RobotRemoved(Option<RobotId>),
    ShowDropdown,
    ChangeFilter(RobotFilter),
    UpdateRobotIdList(RobotId, bool),
}

impl Component for RobotsList {
    type Message = RobotsListMessage;
    type Properties = RobotsListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(RobotsListMessage::FetchRobotsByGroupId);
        RobotsList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_sub: None,
            robot_delete_task: None,
            robot_add_task: None,
            show_dropdown_filter: false,
            filter: RobotFilter::Alls,
            robot_list: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            RobotsListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            RobotsListMessage::FetchRobotsByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = robot_model::robots_list_by_group::Variables {
                        group_id: group_id.0,
                    };

                    let task = robot_model::RobotsListByGroup::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                RobotsListMessage::Robots(response)
                            },
                    );
                    self.robot_sub = Some(task);
                }
            }
            RobotsListMessage::Robots(response) => {
                self.robot_list = response
                    .clone()
                    .and_then(|data| Some(data.robot_group))
                    .unwrap_or(vec![])
                    .iter()
                    .filter(|robots| {

                        self.filter == RobotFilter::Alls && {robots.enabled == true || robots.enabled == false} ||
                
                        self.filter == RobotFilter::Enabled && robots.enabled == true ||
        
                        self.filter == RobotFilter::Disabled && robots.enabled == false
        
                    })
                    .map(|item| {
                        let naive = chrono::NaiveDate::from_ymd_opt(2023, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();

                        let timestamp = item.robot_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive);

                        let robot_type = item.robot_type.clone().unwrap_or(robots_list_by_group::RoboxRobotTypeEnum::Different);

                        let time_fn = get_creation_date(timestamp);

                        RobotProfile { 
                            name: item.robot_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string()), 
                            path: item.robot_profile.clone().and_then(|data| Some(data.path)).unwrap_or("".to_string()), 
                            timestamp: time_fn,
                            robot_id: RobotId(item.robot_profile.clone().and_then(|data| Some(data.robot_id)).unwrap_or(Uuid::default())), 
                            enabled: item.enabled,
                            robot_type,
                        }
                    }).collect();
            }
            RobotsListMessage::AddRobot(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = robot_model::robot_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        robot_id: robot_id.0,
                        robot_type: robot_group_add::RoboxRobotTypeEnum::Different,
                    };

                    let task = robot_model::RobotGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let robot_id = if let Some(robot) = response {
                                robot.insert_robot_group_one.and_then(|data| Some(RobotId(data.robot_id)))
                            } else {
                                None
                            };
                            RobotsListMessage::RobotAdded(robot_id)
                        },
                    );
                    self.robot_add_task = Some(task);
                }
            }
            RobotsListMessage::RemoveRobot(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::robot_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        robot_id: robot_id.0,
                    };

                    let task = robot_model::RobotGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let robot_id = if let Some(response) = response {
                                if response.delete_robot_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(RobotId(response.delete_robot_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].robot_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            RobotsListMessage::RobotRemoved(robot_id)
                        },
                    );
                    self.robot_delete_task = Some(task);
                }
            }
            RobotsListMessage::RobotAdded(robot_id) => {
                if let Some(robot_id) = robot_id {
                    self.robot_list.push(RobotProfile { 
                            name: String::from(""), 
                            path: String::from(""), 
                            timestamp: String::from(""), 
                            robot_id, 
                            enabled: false, 
                            robot_type: robots_list_by_group::RoboxRobotTypeEnum::Different 
                        }
                    );
                } else {
                    should_update = true;
                }
            }
            RobotsListMessage::RobotRemoved(robot_id) => {
                if let Some(robot_id) = robot_id {
                    self.robot_list.retain(|u| u.robot_id != robot_id);
                } else {
                    should_update = true;
                }
            }
            RobotsListMessage::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            RobotsListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                self.link.send_message(RobotsListMessage::FetchRobotsByGroupId);
            }
            RobotsListMessage::UpdateRobotIdList(robot_id, enabled) => {
                for robot in self.robot_list.iter_mut() {
                    if robot.robot_id == robot_id {
                        robot.enabled = enabled;
                    }
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let on_alls = self.link.callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Alls));
        let on_enabled = self.link.callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Enabled));
        let on_disabled = self.link.callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Disabled));
        let on_dropdown = self.link.callback(|_| RobotsListMessage::ShowDropdown);
        let on_change_list = self.link.callback(|(robot_id, enabled)| RobotsListMessage::UpdateRobotIdList(robot_id, enabled));
        let on_robot_delete = self.link.callback(|robot_id| RobotsListMessage::RemoveRobot(robot_id));
        let maybe_option_seleted = match self.filter {
            RobotFilter::Alls => "Everyone",
            RobotFilter::Enabled => "Enabled",
            RobotFilter::Disabled => "Disabled",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };
        let maybe_dropdown_by_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == RobotFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_enabled>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Enabled {true} else {false}} />
                                        <span class={if self.filter == RobotFilter::Enabled {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Enabled")}{"s"}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_disabled>
                                    <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Disabled {true} else {false}} />
                                    <span class={if self.filter == RobotFilter::Disabled {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Disabled")}{"s"}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let robot_list = self
            .robot_list
            .iter()
            .filter(|data| data.robot_type != robots_list_by_group::RoboxRobotTypeEnum::Different)
            .map(|item| {
            let robot_profile = item.clone();
            html! {
                <RobotsCard user_profile=self.props.user_profile.clone()
                    user_id=self.props.user_id.clone()
                    robot_id=item.robot_id.clone()
                    group_id=self.props.group_id.clone()
                    on_app_route={self.props.on_app_route.clone()}
                    on_robot_delete=Some(on_robot_delete.clone())
                    on_change_list=on_change_list.clone()
                    robot_profile={robot_profile} />
            }
        }).collect::<Html>();

        let other_robot_list = self
            .robot_list
            .iter()
            .filter(|data| data.robot_type == robots_list_by_group::RoboxRobotTypeEnum::Different)
            .map(|item| {
            let robot_profile = item.clone();
            html! {
                <RobotsCard user_profile=self.props.user_profile.clone()
                    user_id=self.props.user_id.clone()
                    robot_id=item.robot_id.clone()
                    group_id=self.props.group_id.clone()
                    on_app_route={self.props.on_app_route.clone()}
                    on_robot_delete=Some(on_robot_delete.clone())
                    on_change_list=on_change_list.clone()
                    robot_profile={robot_profile} />
            }
        }).collect::<Html>();

        let maybe_robot_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    RobotSelectOption::Robot(robot_id) => RobotsListMessage::AddRobot(robot_id),
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <RobotSelect on_select=on_select 
                            allow_create=true
                            group_id=self.props.group_id
                            user_profile=self.props.user_profile.clone()
                            on_app_route=self.props.on_app_route.clone() />
                    })
                } else {
                    Some(html! {
                        <SearchRobotdGroup on_app_route=self.props.on_app_route.clone()
                            group_id=self.props.group_id
                            user_id=None />
                    })
                }
            })
            .unwrap_or(html! {});
        let group_id = self.props.group_id; 
        let on_direct_meet = self.link.callback(move |_| RobotsListMessage::AppRoute(AppRoute::MeetDirect(group_id)));
        let maybe_meet = {
            html! {
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                       <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
            }
        };
        let maybe_user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
            
        let head_section = html! {
            <div class="d-flex flex-wrap align-items-lg-center justify-content-between mb-md-5 mb-lg-6">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <div class="d-flex flex-wrap justify-content-between align-items-center col-12 col-xl-5 mb-4 mb-lg-0">
                    {maybe_meet}
                    {maybe_robot_search}
                    {maybe_user_profile_pic}
                </div>
            </div>
        };
        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Robots")} <span class="ps-1">{"("}{self.robot_list.iter().filter(|item | item.robot_type != robots_list_by_group::RoboxRobotTypeEnum::Different).count()}{")"}</span>
                </span>
                {maybe_dropdown_by_user}
            </div>
        };

        let maybe_option = if self.robot_list.iter().filter(|item | item.robot_type != robots_list_by_group::RoboxRobotTypeEnum::Different).count() > 0 {
            html! {
                <div class="d-flex flex-wrap mb-3">
                    {robot_list}
                </div>
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No robots here.")}</span>
                </div>
            }
        };
        let maybe_option_two = if self.robot_list.iter().filter(|item | item.robot_type == robots_list_by_group::RoboxRobotTypeEnum::Different).count() > 0 {
            html! {
                <div class="d-flex flex-wrap pt-5">
                    {other_robot_list}
                </div>
            }
        } else {
            html! {}
        };

        html! { 
            <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7">
                {head_section}
                {maybe_dropdown}
                {maybe_option}
                {   
                    if self.robot_list.iter().filter(|item | item.robot_type == robots_list_by_group::RoboxRobotTypeEnum::Different).count() > 0 {
                        html! {
                            <>
                                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                    {lang::dict("Other Robots")} <span class="ps-1">{"("}{self.robot_list.iter().filter(|item | item.robot_type == robots_list_by_group::RoboxRobotTypeEnum::Different).count()}{")"}</span>
                                </span>
                                {maybe_option_two}
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
         }
    }
}