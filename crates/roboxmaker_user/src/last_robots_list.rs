use log::*;
use yew::prelude::*;
use crate::last_robots_card::UserStyle;
use crate::last_robots_card::LastRobotCard;
use yew::{html, Component, Html, Properties};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_types::types::{RobotId, UserId, GroupId, MyUserProfile};


pub struct RobotListByUser {}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotListByUserProperties {
    pub robots: Vec<RobotId>,
    pub allow_edit: bool,
    #[prop_or(None)]
    pub group_id: Option<GroupId>,
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_id: Option<UserId>,
    #[prop_or(None)]
    pub on_list_change: Option<Callback<()>>,
    pub maybe_style: UserStyle,
}

#[derive(Debug)]
pub enum RobotListByUserMessage {
}

impl Component for RobotListByUser {
    type Message = RobotListByUserMessage;
    type Properties = RobotListByUserProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        RobotListByUser {
            // link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        // match msg {

        // }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;
        
        if ctx.props() != old_props {
            should_render = true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user_profile = ctx.props().user_profile.clone();
        let maybe_robot_space = |robot_id: &RobotId| {
            html! {
                <div class="pb-4 pe-5">
                    <LastRobotCard user_profile={user_profile.clone()}
                        user_id={ctx.props().user_id.clone()}
                        robot_id={robot_id.clone()}
                        group_id={ctx.props().group_id.clone()}
                        maybe_style={ctx.props().maybe_style.clone()}
                        />
                </div>
            }
        };
        let maybe_robot_member = |robot_id: &RobotId| {
            html! {
                <div class="pb-4">
                    <LastRobotCard user_profile={user_profile.clone()}
                        user_id={ctx.props().user_id.clone()}
                        robot_id={robot_id.clone()}
                        group_id={ctx.props().group_id.clone()}
                        maybe_style={ctx.props().maybe_style.clone()}
                        />
                </div>
            }
        };

        let maybe_robots = {
            if ctx.props().robots.len() > 0 {
                let maybe_robot_style = {
                    match ctx.props().maybe_style {
                        UserStyle::MySpace => {
                            html! {
                                <div class="d-flex flex-row">
                                    {
                                        ctx.props().robots.iter().map(|robot_id| {
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
                                        ctx.props().robots.iter().map(|robot_id| {
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
                                        ctx.props().robots.iter().map(|robot_id| {
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