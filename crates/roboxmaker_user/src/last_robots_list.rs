use log::*;
use yew::prelude::*;
use crate::last_robots_card::UserStyle;
use crate::last_robots_card::LastRobotCard;
use yew::{html, Component, ComponentLink, Html, ShouldRender, Properties};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_types::types::{RobotId, UserId, GroupId, AppRoute, MyUserProfile};


pub struct RobotListByUser {
    props: RobotListByUserProperties,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotListByUserProperties {
    pub robots: Vec<RobotId>,
    pub allow_edit: bool,
    pub group_id: Option<GroupId>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_id: Option<UserId>,
    pub on_app_route: Callback<AppRoute>,
    pub on_list_change: Option<Callback<()>>,
    pub maybe_style: UserStyle,
}

#[derive(Debug)]
pub enum RobotListByUserMessage {
}

impl Component for RobotListByUser {
    type Message = RobotListByUserMessage;
    type Properties = RobotListByUserProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        RobotListByUser {
            // link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        // match msg {

        // }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let user_profile = self.props.user_profile.clone();
        let maybe_robot_space = |robot_id: &RobotId| {
            html! {
                <div class="pb-4 pe-5">
                    <LastRobotCard user_profile=user_profile.clone()
                        user_id=self.props.user_id.clone()
                        robot_id=robot_id.clone()
                        group_id=self.props.group_id.clone()
                        on_app_route={self.props.on_app_route.clone()}
                        maybe_style=self.props.maybe_style.clone()
                        />
                </div>
            }
        };
        let maybe_robot_member = |robot_id: &RobotId| {
            html! {
                <div class="pb-4">
                    <LastRobotCard user_profile=user_profile.clone()
                        user_id=self.props.user_id.clone()
                        robot_id=robot_id.clone()
                        group_id=self.props.group_id.clone()
                        on_app_route={self.props.on_app_route.clone()}
                        maybe_style=self.props.maybe_style.clone()
                        />
                </div>
            }
        };

        let maybe_robots = {
            if self.props.robots.len() > 0 {
                let maybe_robot_style = {
                    match self.props.maybe_style {
                        UserStyle::MySpace => {
                            html! {
                                <div class="d-flex flex-row">
                                    {
                                        self.props.robots.iter().map(|robot_id| {
                                            maybe_robot_space(robot_id)
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        }
                        UserStyle::MemberProfile => {
                            html! {
                                <div class="container-robots-card-class-2">
                                    {
                                        self.props.robots.iter().map(|robot_id| {
                                            maybe_robot_member(robot_id)
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        }
                        UserStyle::ListHome => {
                            html! {
                                <div class="container-robots-card-class mt-4">
                                    {
                                        self.props.robots.iter().map(|robot_id| {
                                            maybe_robot_member(robot_id)
                                        }).collect::<Html>()
                                    }
                                </div>
                            }

                        },
                    }
                };
                html! {
                    <>
                        {maybe_robot_style}
                    </>
                }
            } else {
                html! {
                    <div class="text-center">
                        <p class="is-size-7-mobile is-size-5-tablet is-size-4-desktop">{lang::dict("No robots here.")}</p>
                    </div>
                }
            }
        };

        html! { maybe_robots }
    }
}