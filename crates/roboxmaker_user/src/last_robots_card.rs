use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html, Properties};

use roboxmaker_models::robot_model;
use roboxmaker_main::{config, lang};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{RobotId, UserId, GroupId, AppRoute, MyUserProfile};
use roboxmaker_loaders::placeholders::last_robots_placeholder::LastRobotsPlaceholder;
use roboxmaker_loaders::placeholders::card_robot_my_space::CardRobotMySpacePlaceholder;
use roboxmaker_loaders::placeholders::card_user_robots_placeholder::UserRobotsPlaceholder;

pub struct LastRobotCard {
    graphql_task: Option<GraphQLTask>,
    robot_task: Option<RequestTask>,
    robot: Option<robot_model::robot_by_id::RobotByIdRobotByPk>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserStyle {
    MySpace,
    MemberProfile,
    ListHome,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LastRobotCardProperties {
    pub user_profile: Option<MyUserProfile>,
    pub user_id: Option<UserId>,
    pub robot_id: RobotId,
    pub group_id: Option<GroupId>,
    #[prop_or(None)]
    pub on_app_route: Option<Callback<AppRoute>>,
    pub maybe_style: UserStyle,
}

#[derive(Debug)]
pub enum LastRobotCardMessage {
    FetchRobotById(RobotId),
    Robot(Option<robot_model::robot_by_id::ResponseData>),
    AppRoute(AppRoute),
}

impl Component for LastRobotCard {
    type Message = LastRobotCardMessage;
    type Properties = LastRobotCardProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(LastRobotCardMessage::FetchRobotById(ctx.props().robot_id));
        LastRobotCard {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_task: None,
            robot: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            LastRobotCardMessage::FetchRobotById(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::robot_by_id::Variables {
                        robot_id: robot_id.0,
                    };

                    let task = robot_model::RobotById::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                LastRobotCardMessage::Robot(response)
                            },
                    );
                    self.robot_task = Some(task);
                }
            }
            LastRobotCardMessage::Robot(robot) => {
                self.robot = robot.clone().and_then(|data| data.robot_by_pk)
            }
            LastRobotCardMessage::AppRoute(route) => {
                if let Some(on_app_route) = &ctx.props().on_app_route {
                    on_app_route.emit(route);
                }
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        let mut should_render = false;
        if ctx.props().robot_id != old_props.robot_id {
            should_render = true;
            ctx.link()
                .send_message(LastRobotCardMessage::FetchRobotById(ctx.props().robot_id));
        }

        if ctx.props() != old_props {
            should_render = true;
        }

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let robot_id = ctx.props().robot_id;
        let group_id = if let Some(group_id) = ctx.props().group_id {
            group_id
        } else {
            GroupId(Uuid::default())
        };
        let user_id = if let Some(user_id) = ctx.props().user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };

        let on_robot = ctx.link().callback(move |_| {
            LastRobotCardMessage::AppRoute(AppRoute::Robot{robot_id, group_id, user_id})
        });
        let maybe_placeholder = {
            match ctx.props().maybe_style {
                UserStyle::MySpace => {
                    html! {
                        <CardRobotMySpacePlaceholder />
                    }
                }
                UserStyle::MemberProfile => {
                    html! {
                        <UserRobotsPlaceholder />
                    }
                }
                UserStyle::ListHome => {
                    html! {
                        <LastRobotsPlaceholder />
                    }
                },
            }
        };
        let robots_list_by_user_id = self
            .robot
            .as_ref()
            .and_then(|robot| robot.robot_profile.as_ref())
            .and_then(|robot_profile| {
                let robot_thumb = format!(
                    "{}/robots/{}_thumbnail.jpg",
                    config::AKER_FILES_URL,
                    robot_profile.path
                );
                let maybe_style_robot = {
                    match ctx.props().maybe_style {
                        UserStyle::MySpace => {
                            html! {
                                <div class="card-robots-mp p-5 d-flex flex-row align-items-center" style="display: grid;">
                                    <img class="img-card-72" src={robot_thumb} alt="image of robot" />
                                    <div class="d-flex flex-column w-100 ms-3">
                                        <div class="d-flex flew-wrap justify-content-between">
                                            <strong class="text-white noir-bold is-size-18 lh-22">{&robot_profile.name}</strong>
                                            <a onclick={&on_robot}>
                                                <img src="/icons/play.svg" style="height: 22px;" />
                                            </a>
                                        </div>
                                        <div class="d-flex justify-content-between text-white noir-normal is-size-14 lh-17 mb-1 mt-2">
                                            <span>{lang::dict("Progress")}</span>
                                            <span>{"25%"}</span>
                                        </div>
                                        <progress class="progress mt-1 w-100" value="20" max="100" style="border-radius: 10px !important;"></progress>
                                    </div>
                                </div>
                            }
                        }
                        UserStyle::MemberProfile => {
                            html! {
                                <div class="card-robots-user p-4" style="display: grid;">
                                    <article class="media d-flex align-items-center">
                                        <a onclick={&on_robot}>
                                            <img class="img-card-72" src={robot_thumb} alt="image of robot" />
                                        </a>
                                        <a class="w-100 ms-2" onclick={&on_robot}>
                                            <div class="content d-flex justify-content-between">
                                                <p>
                                                    <strong class="text-white noir-bold is-size-18 lh-22">{&robot_profile.name}</strong>
                                                </p>
                                            </div>
                                            <div class="d-flex justify-content-between text-white noir-normal is-size-14 lh-17 mb-1">
                                                <span>{lang::dict("Progress")}</span>
                                                <span>{"25%"}</span>
                                            </div>
                                            <progress class="progress mt-1 w-100" value="20" max="100" style="border-radius: 10px !important;"></progress>
                                        </a>
                                    </article>
                                </div>
                            }
                        }
                        UserStyle::ListHome => {
                            html! {
                                <div class="card-robots-class-2 bg-white px-4" style="display: grid;">
                                    <div class="d-flex justify-content-between align-items-center">
                                        <a onclick={&on_robot}>
                                            <img class="img-card-64" src={robot_thumb} alt="image of robot" />  
                                        </a>
                                        <div class="d-flex flex-column w-100 ms-3">
                                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{&robot_profile.name}</span>
                                            <div class="d-flex justify-content-between text-primary-blue-dark noir-normal is-size-14 lh-17 my-3">
                                                <span>{lang::dict("Progress")}</span>
                                                <span>{"25%"}</span>
                                            </div>
                                            <progress class="progress w-100" value="20" max="100" style="border: 1px solid #022754; border-radius: 10px !important;"></progress>
                                        </div>
                                    </div>
                                </div>
                            }
                        },
                    }
                };
                Some(html! {
                    {maybe_style_robot}
                })
            })
            .unwrap_or(html! {
                maybe_placeholder
            });
        html! {
            {robots_list_by_user_id}
        }
    }
}