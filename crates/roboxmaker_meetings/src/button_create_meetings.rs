use log::*;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_types::types::{GroupId, AppRoute};

pub struct ButtonCreateMeetings {
    link: ComponentLink<Self>,
    props: ButtonCreateMeetingsProperties,
    show_create: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ButtonCreateMeetingsProperties {
    pub on_select: Callback<()>,
    pub allow_create: bool,
    pub group_id: Option<GroupId>,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub title: String,
    pub schedule_time: String,
    pub start_of_meeting: String,
    pub end_of_meeting: String,
    pub met_invalid: bool,
}

#[derive(Debug)]
pub enum ButtonCreateMeetingsMessage {
    SelectMeetings,
}

impl Component for ButtonCreateMeetings {
    type Message = ButtonCreateMeetingsMessage;
    type Properties = ButtonCreateMeetingsProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonCreateMeetings {
            link,
            props,
            show_create: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ButtonCreateMeetingsMessage::SelectMeetings => {
                self.show_create = false;
                self.props.on_select.emit(());
            }
        }
        should_render
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
        let on_select = self
            .link
            .callback(move |_| ButtonCreateMeetingsMessage::SelectMeetings);
        let disable_button = if !self.props.title.is_empty() && !self.props.schedule_time.is_empty() && 
            !self.props.start_of_meeting.is_empty() && !self.props.end_of_meeting.is_empty() && self.props.met_invalid == false {
            false
        } else {
            true
        };
        let class_btn = if !self.props.title.is_empty() && !self.props.schedule_time.is_empty() && 
            !self.props.start_of_meeting.is_empty() && !self.props.end_of_meeting.is_empty() && self.props.met_invalid == false {
            "button-meeting-create bg-primary-blue-dark d-flex align-items-center justify-content-center"
        } else {
            "button-meeting-create-disabled bg-primary-blue-dark d-flex align-items-center justify-content-center opacity-75"
        };
        html! {
            <>
                <button class=class_btn disabled=disable_button
                    onmousedown=on_select.clone()>
                    <img src="/icons/video.svg" style="height: 20px;" />
                    <span class="text-white text-center noir-bold is-size-16 lh-20" style="margin-left: 8px;">{lang::dict("Start Meeting")}</span>
                </button>
            </>
        }
    }
}