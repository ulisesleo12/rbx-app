use log::*;
use uuid::Uuid;
use yew::prelude::*;
use std::time::Duration;
use code_location::code_location;
use crate::robots_list::RobotProfile;
use yew::services::{TimeoutService, Task};
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use roboxmaker_main::{lang, config};
use roboxmaker_models::robot_model::{self, robots_list_by_group};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_loaders::placeholders::card_robot_list::CardRobotListPlaceholder;
use roboxmaker_types::types::{RobotId, GroupId, UserId, AppRoute, MyUserProfile};

pub struct RobotsCard {
    link: ComponentLink<Self>,
    props: RobotsCardProperties,
    graphql_task: Option<GraphQLTask>,
    robot_update_task: Option<RequestTask>,
    maybe_placeholder: bool,
    job: Option<Box<dyn Task>>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotsCardProperties {
    pub robot_id: RobotId,
    pub user_profile: Option<MyUserProfile>,
    pub user_id: Option<UserId>,
    pub group_id: GroupId,
    pub on_app_route: Option<Callback<AppRoute>>,
    pub on_robot_delete: Option<Callback<RobotId>>,
    pub on_change_list: Callback<(RobotId, bool)>,
    pub robot_profile: RobotProfile,
}

#[derive(Debug)]
pub enum RobotsCardMessage {
    AppRoute(AppRoute),
    EnabledToggle(Option<robot_model::update_robot_group_enabled::ResponseData>),
    SaveRobot(RobotId),
    HiddenPlaceholder,
    DeleteRobot,
}

impl Component for RobotsCard {
    type Message = RobotsCardMessage;
    type Properties = RobotsCardProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let handle = TimeoutService::spawn(
            Duration::from_millis(400),
            link.callback(|_| RobotsCardMessage::HiddenPlaceholder),
        );

        RobotsCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_update_task: None,
            maybe_placeholder: true,
            job: Some(Box::new(handle)),
        }
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            RobotsCardMessage::AppRoute(route) => {
                if let Some(on_app_route) = &self.props.on_app_route {
                    on_app_route.emit(route);
                }
            }
            RobotsCardMessage::EnabledToggle(response) => {
                let robot_id = self.props.robot_id;
                if response.clone().and_then(|data| data.update_robot_group_by_pk).clone().and_then(|update_robot_group_by_pk| Some(update_robot_group_by_pk.enabled)).is_some() {
                    self.props.robot_profile.enabled = response.and_then(|data| data.update_robot_group_by_pk).clone().and_then(|update_robot_group_by_pk| Some(update_robot_group_by_pk.enabled)).unwrap_or(false);
                    self.props.on_change_list.emit((robot_id, self.props.robot_profile.enabled))
                }
            }
            RobotsCardMessage::SaveRobot(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::update_robot_group_enabled::Variables {
                        robot_id: robot_id.0,
                        group_id: self.props.group_id.0,
                        enabled: !self.props.robot_profile.enabled,
                    };

                    let task = robot_model::UpdateRobotGroupEnabled::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                RobotsCardMessage::EnabledToggle(response)
                            },
                    );
                    self.robot_update_task = Some(task);
                }
            }
            RobotsCardMessage::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
            RobotsCardMessage::DeleteRobot => {
                let robot_id = self.props.robot_id;
                if let Some(robot) = &self.props.on_robot_delete {
                    robot.emit(robot_id)
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
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        let _none = self.job.as_ref();

        let robot_id = self.props.robot_id;
        let group_id = self.props.group_id;
        let user_id = if let Some(user_id) = self.props.user_id {
            user_id
        } else {
            UserId(Uuid::default())
        };

        let on_robot = self.link.callback(move |_| {
            RobotsCardMessage::AppRoute(AppRoute::Robot(robot_id, group_id, user_id))
        });
        
        let on_enabled_toggle = self.link.callback(move |_| RobotsCardMessage::SaveRobot(robot_id));        

        let text_toggle = if self.props.robot_profile.enabled {
            html! {
                <span class="text-white noir-bold is-size-16 lh-19">{lang::dict("Enabled")}</span>
            }
        } else {
            html! {
                <span class="text-white noir-bold is-size-16 lh-19">{lang::dict("Disabled")}</span>
            }
        };

        let icon_robot_hidden = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_student.is_some() {
                    Some(html! {
                        <a onclick=&on_robot>
                            <img src="/icons/play.svg" style="height: 22px;" />
                        </a>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let maybe_option_robot_view = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <div class="field d-flex flex-wrap align-items-center ps-2">
                                <div class="control pe-2" style="padding-top: 5px;">
                                    <label class="switch">
                                        <input type="checkbox" checked=self.props.robot_profile.enabled
                                            onclick=on_enabled_toggle />
                                        <span class="slider round"></span>
                                    </label>
                                </div>
                                {text_toggle}
                            </div>
                        </>
                    })
                } else {
                    Some(html! {
                        <span class="text-white noir-light is-size-12 lh-17 text-nowrap" style="padding-top: 14px;">
                            <span class="is-size-12">
                                <i class="far fa-clock"></i>
                            </span>
                            <span class="px-2">{lang::dict("Added")} { &self.props.robot_profile.timestamp}</span>
                        </span>
                        // <div class="d-flex flex-column">
                        //     <progress class="progress is-info" style="height: 18px; width: 145px; border-radius: 10px;" value="20" max="100"></progress>
                        // </div>
                    })
                }
            })
            .unwrap_or(html! {});
 
        let robot_thumb = format!(
            "{}/robots/{}_thumbnail.jpg",
            config::AKER_FILES_URL,
            self.props.robot_profile.path
        );
        let on_delete_robot = self.link.callback(move |_| RobotsCardMessage::DeleteRobot);

        let maybe_delete = if self.props.robot_profile.robot_type == robots_list_by_group::RoboxRobotTypeEnum::Different {
            html! {
                <button class="btn text-danger border border-0 p-0" onclick={on_delete_robot}>
                    <i class="fas fa-trash-alt fa-lg"></i>
                </button>
            }
        } else {
            html! {}
        };

        let maybe_robots = if self.maybe_placeholder {
            html! {
                <CardRobotListPlaceholder />
            }
        } else {
            html! {
                <div class="card-robot-view-degree d-flex justify-content-between p-4 mb-3 mb-lg-5 me-md-3 me-lg-5">
                    <div class="d-flex align-items-center">
                        <a onclick=&on_robot>
                            <img src=robot_thumb class="img-card-64" />
                        </a>
                        <div class="d-flex flex-column justify-content-between ms-2">
                            <span class="text-white noir-bold is-size-18 lh-22 mb-4">{&self.props.robot_profile.name}</span>
                            {maybe_option_robot_view}
                            // <span class="text-white noir-light is-size-12 lh-17 text-nowrap"
                            //     style="padding-top: 14px;">
                            //     <span class="is-size-12">
                            //         <i class="far fa-clock"></i>
                            //     </span>
                            //     <span class="px-2">{lang::dict("Added")} { &self.props.robot_profile.timestamp}</span>
                            // </span>
                        </div>
                    </div>
                    <div class="d-flex flex-column justify-content-between align-items-end">
                        {icon_robot_hidden}
                        {maybe_delete}
                    </div>
                </div>
            }
        };

        html! {
            maybe_robots
        }
    }
}
