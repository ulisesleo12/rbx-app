use log::*;
use yew::prelude::*;
use yew::{html, Component, Html};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_types::types::GroupId;

pub struct ButtonCreateMeetings {
    show_create: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ButtonCreateMeetingsProperties {
    pub on_select: Callback<()>,
    pub allow_create: bool,
    pub group_id: Option<GroupId>,
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

    fn create(_ctx: &Context<Self>) -> Self {
        ButtonCreateMeetings {
            show_create: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ButtonCreateMeetingsMessage::SelectMeetings => {
                self.show_create = false;
                ctx.props().on_select.emit(());
            }
        }
        should_render
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
        let on_select = ctx
            .link()
            .callback(move |_| ButtonCreateMeetingsMessage::SelectMeetings);
        let disable_button = if !ctx.props().title.is_empty() && !ctx.props().schedule_time.is_empty() && 
            !ctx.props().start_of_meeting.is_empty() && !ctx.props().end_of_meeting.is_empty() && ctx.props().met_invalid == false {
            false
        } else {
            true
        };
        let class_btn = if !ctx.props().title.is_empty() && !ctx.props().schedule_time.is_empty() && 
            !ctx.props().start_of_meeting.is_empty() && !ctx.props().end_of_meeting.is_empty() && ctx.props().met_invalid == false {
            "button-meeting-create bg-primary-blue-dark d-flex align-items-center justify-content-center"
        } else {
            "button-meeting-create-disabled bg-primary-blue-dark d-flex align-items-center justify-content-center opacity-75"
        };
        html! {
            <>
                <button class={class_btn} disabled={disable_button}
                    onclick={on_select.clone()}>
                    <img src="/icons/video.svg" style="height: 20px;" />
                    <span class="text-white text-center noir-bold is-size-16 lh-20" style="margin-left: 8px;">{lang::dict("Start Meeting")}</span>
                </button>
            </>
        }
    }
}