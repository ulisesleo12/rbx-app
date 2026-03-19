use log::*;
use yew::prelude::*;
use crate::ActivityStyle;
use yew::{html, Component, Html};
use crate::activity_list::ActivityProfile;

use roboxmaker_main::lang;
use roboxmaker_types::types::{ClassesId, GroupId};

pub struct ActivityCardClasses {}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ActivityCardClassesProps {
    pub activity_profile: Option<ActivityProfile>,
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub maybe_style: ActivityStyle,
}

#[derive(Debug)]
pub enum ActivityCardClassesMessage {}

impl Component for ActivityCardClasses {
    type Message = ActivityCardClassesMessage;
    type Properties = ActivityCardClassesProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ActivityCardClasses {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        match msg {
            
        }
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
        
        if let Some(activity_profile) = &ctx.props().activity_profile {
            let maybe_score = if activity_profile.score < 1  {
                html! {
                }
            } else {
                html! {
                    <span class="text-gray-purple-two noir-light is-size-14 lh-17 ps-2">{" | "}{&activity_profile.score}{" ptos"}</span>
                }
            };
            let style = {
                match ctx.props().maybe_style {
                    ActivityStyle::ClassesPage => {
                        html! { }
                    }
                    ActivityStyle::ClassesCard => {
                        html! {
                            <>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center py-4">
                                    <img src="/icons/clipboard-2.svg" style="height: 22px;" />
                                    <span class="ps-2">{&activity_profile.title}</span>
                                </span>
                                <div class="d-flex flex-wrap align-items-center">
                                    <span class="text-gray-purple-two noir-light is-size-14 lh-17 ps-5 ms-4">{lang::dict("Until ")}{&activity_profile.deliver}</span>
                                    {maybe_score}
                                </div>
                            </>
                        }
                    }
                }
            };
            html! {
                style
            }
        } else {
            html! {}
        }
    }
}