use log::*;
use std::vec;
use yew::prelude::*;
use crate::ActivityStyle;
use crate::activity_list::ActivityProfile;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_types::types::{ClassesId, GroupId};

pub struct ActivityCardClasses {
    props: ActivityCardClassesProps,
}

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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ActivityCardClasses {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        match msg {
            
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = true;

        if self.props != props {
            self.props = props;
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        
        if let Some(activity_profile) = &self.props.activity_profile {
            let maybe_score = if activity_profile.score < 1  {
                html! {
                }
            } else {
                html! {
                    <span class="text-gray-purple-two noir-light is-size-14 lh-17 ps-2">{" | "}{&activity_profile.score}{" ptos"}</span>
                }
            };
            let style = {
                match self.props.maybe_style {
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