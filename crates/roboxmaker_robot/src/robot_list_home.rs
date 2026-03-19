use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_main::config;
use roboxmaker_loaders::placeholders::card_robots_placeholder::CardRobotsPlaceholder;
use roboxmaker_types::types::{GroupId, UserId, AppRoute, MyUserProfile, RobotProfile};

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

pub struct RobotListHome {
    robot_list: Vec<RobotProfile>,
    list_robots_state: LoadRobots,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotListHomeProps {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub user_id: Option<UserId>,
    pub robot_list: Vec<RobotProfile>,
}

#[derive(Debug)]
pub enum RobotListHomeMessage {
    FetchRobotsByGroupId,
}

impl Component for RobotListHome {
    type Message = RobotListHomeMessage;
    type Properties = RobotListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(RobotListHomeMessage::FetchRobotsByGroupId);
        RobotListHome {
            robot_list: vec![],
            list_robots_state: LoadRobots::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            RobotListHomeMessage::FetchRobotsByGroupId => {
                self.list_robots_state = LoadRobots::Loading;

                self.robot_list = ctx.props().robot_list.clone();

                if !self.robot_list.is_empty() {
                    self.list_robots_state = LoadRobots::Load(LoadRobotsFound::Found);
                } else {
                    self.list_robots_state = LoadRobots::Load(LoadRobotsFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().robot_list != old_props.robot_list {
            ctx.link().send_message(RobotListHomeMessage::FetchRobotsByGroupId);
        }
        
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let user_id = if let Some(user_id) = ctx.props().user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };

        let card_robots_list = self.robot_list.iter().map(|item| {
            let robot_id = item.robot_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_robot = Callback::from(move |_| navigator.push(&AppRoute::Robot{robot_id, group_id, user_id}));

            let robot_thumb = format!(
                "{}/robots/{}_thumbnail.jpg",
                config::AKER_FILES_URL,
                item.path
            );
            html! {
                <div class="card-robot-view d-flex d-align-items-center p-5 me-5">
                    <div class="d-flex align-items-center">
                        <a onclick={&on_robot}>
                            <img src={robot_thumb} class="img-card-64" />
                        </a>
                        <div class="d-flex flex-column ms-2">
                            <a onclick={&on_robot}>
                                <span class="text-white noir-medium is-size-18 lh-22 ">{&item.name}</span>
                            </a>
                            <span class="text-gray-blue noir-light is-size-14 lh-17 d-flex d-align-items-center"
                                style="padding-top: 14px; padding-bottom: 14px;">
                                <span class="is-size-14">
                                    <i class="far fa-clock"></i>
                                </span>
                                <div class="d-flex flex-wrap">
                                    <span class="px-2">{lang::dict("Added")}</span>
                                    <span>{&item.timestamp}</span>
                                </div>
                            </span>
                            <div class="d-flex justify-content-between text-white noir-normal is-size-14 lh-17 mb-1">
                                <span>{lang::dict("Progress")}</span>
                                <span>{"25%"}</span>
                            </div>
                            <div class="progress progress-home">
                                <div class="progress-bar" role="progressbar" style="width: 25%; height: 14px;" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100"></div>
                            </div>
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