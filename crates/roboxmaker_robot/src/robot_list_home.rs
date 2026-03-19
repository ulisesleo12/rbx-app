use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_main::config;
use roboxmaker_models::{school_model, robot_model};
use roboxmaker_utils::funtions::get_creation_date_robot;
use roboxmaker_types::types::{GroupId, RobotId, UserId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_loaders::placeholders::card_robots_placeholder::CardRobotsPlaceholder;

#[derive(Debug, Clone)]
enum LoadRobotsFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadRobots {
    Loading,
    Load(LoadRobotsFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RobotProfile {
    pub name: String,
    pub timestamp: String,
    pub path: String,
    pub robot_id: RobotId,
}

pub struct RobotListHome {
    link: ComponentLink<Self>,
    props: RobotListHomeProps,
    graphql_task: Option<GraphQLTask>,
    robots_list_task: Option<SubscriptionTask>,
    robot_list: Vec<RobotProfile>,
    list_robots_state: LoadRobots,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotListHomeProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub user_id: Option<UserId>,
}

#[derive(Debug)]
pub enum RobotListHomeMessage {
    AppRoute(AppRoute),
    FetchRobotsByGroupId,
    Robots(Option<robot_model::robots_by_group_id::ResponseData>),
}

impl Component for RobotListHome {
    type Message = RobotListHomeMessage;
    type Properties = RobotListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(RobotListHomeMessage::FetchRobotsByGroupId);
        RobotListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robots_list_task: None,
            robot_list: vec![],
            list_robots_state: LoadRobots::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            RobotListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            RobotListHomeMessage::FetchRobotsByGroupId => {
                self.list_robots_state = LoadRobots::Loading;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = robot_model::robots_by_group_id::Variables {
                        group_id: group_id.0,
                        limit: 10,
                    };

                    let task = robot_model::RobotsByGroupId::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                RobotListHomeMessage::Robots(response)
                            },
                    );
                    self.robots_list_task = Some(task);
                }
            }
            RobotListHomeMessage::Robots(response) => {
                self.robot_list = response
                    .clone()
                    .and_then(|data| Some(data.robot_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|robot_profile| {
                        let name = robot_profile.name.clone();
                        let path = robot_profile.path.clone();
                        let robot_id = robot_profile.robot_id;

                        let timestamp = robot_profile.timestamp;
                        
                        let time_fn = get_creation_date_robot(timestamp);

                        RobotProfile {
                            name: name,
                            timestamp: time_fn,
                            path: path,
                            robot_id: RobotId(robot_id),
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.robot_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_robots_state = LoadRobots::Load(LoadRobotsFound::Found);
                } else {
                    self.list_robots_state = LoadRobots::Load(LoadRobotsFound::NotFound);
                }
            },
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(RobotListHomeMessage::FetchRobotsByGroupId);
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let user_id = if let Some(user_id) = self.props.user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };

        let card_robots_list = self.robot_list.iter().map(|item| {
            let robot_id = item.robot_id;
            let on_robot = self.link.callback(move |_| {
                RobotListHomeMessage::AppRoute(AppRoute::Robot(robot_id, group_id, user_id))
            });
            let robot_thumb = format!(
                "{}/robots/{}_thumbnail.jpg",
                config::AKER_FILES_URL,
                item.path
            );
            html! {
                <div class="card-robot-view d-flex d-align-items-center p-5 me-5">
                    <div class="d-flex align-items-center">
                        <a onclick=&on_robot>
                            <img src=robot_thumb class="img-card-64" />
                        </a>
                        <div class="d-flex flex-column ms-2">
                            <a onclick=&on_robot>
                                <span class="text-white noir-medium is-size-18 lh-22 ">{&item.name}</span>
                            </a>
                            <span class="text-gray-blue noir-light is-size-14 lh-17 text-nowrap"
                                style="padding-top: 14px;">
                                <span class="is-size-14">
                                    <i class="far fa-clock"></i>
                                </span>
                                // <div class="d-flex flex-nowrap">
                                    <span class="px-2">{lang::dict("Added")} { &item.timestamp}</span>
                                    // <span></span>
                                // </div>
                            </span>
                            // <div class="d-flex justify-content-between text-white noir-normal is-size-14 lh-17 mb-1">
                            //     <span>{lang::dict("Progress")}</span>
                            //     <span>{"25%"}</span>
                            // </div>
                            // <div class="progress progress-home">
                            //     <div class="progress-bar" role="progressbar" style="width: 25%; height: 14px;" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100"></div>
                            // </div>
                        </div>
                    </div>
                </div>
            }
        }).collect::<Html>();
        let robot_list = match self.list_robots_state {
            LoadRobots::Loading => {
                html! {
                    <>
                        <CardRobotsPlaceholder />
                        <CardRobotsPlaceholder />
                        <CardRobotsPlaceholder />
                        <CardRobotsPlaceholder />
                        <CardRobotsPlaceholder />
                    </>
                }
            },
            LoadRobots::Load(LoadRobotsFound::Found) => {
                html! {
                    {card_robots_list}
                }
            },
            LoadRobots::Load(LoadRobotsFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No robots here.")}</p>
                    </div>
                }
            },
        };

        html! {
            <div class="d-flex flex-row">   
                {robot_list}
            </div>
        }
    }
}