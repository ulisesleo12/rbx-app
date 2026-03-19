use log::*;
use uuid::Uuid;
use yew::prelude::*;
use chrono::NaiveDate;
use crate::ClassGroupMeetings;
use yew::services::fetch::FetchTask;
// use chrono::{Local, DateTime, TimeZone, Utc};
use crate::button_create_meetings::MeetSelectButton;
use crate::button_create_meetings::ButtonCreateMeetings;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, meetings_model};
use roboxmaker_types::types::{SchoolId, GroupId, AppRoute, MeetingsId};

pub struct EditMeet {
    link: ComponentLink<Self>,
    props: EditMeetProperties,
    task_add: Option<FetchTask>,
    title: String,
    schedule_time: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct EditMeetProperties {
    pub meetings: Vec<ClassGroupMeetings>,
    pub allow_edit: bool,
    pub inventory_group_id: Uuid,
    pub group_id: GroupId,
    pub class_name: String,
    pub school_id: Option<SchoolId>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub on_list_change: Option<Callback<()>>,
    pub close_modal_callback_meet: Callback<bool>,
    pub close_modal_callback_failed: Callback<bool>,
}

#[derive(Debug)]
pub enum EditMeetMessage {
    FetchListSchools,
    
    Title(String),
    Duration(String),
}

impl Component for EditMeet {
    type Message = EditMeetMessage;
    type Properties = EditMeetProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        EditMeet {
            link,
            props,
            task_add: None,
            title: String::from(""),
            schedule_time: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            EditMeetMessage::Title(title) => {
                self.title = title;
            }
            EditMeetMessage::Duration(schedule_time) => {
                self.schedule_time = schedule_time;
            }
        }
        should_update
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
        let on_title = self.link.callback(|data: InputData| EditMeetMessage::Title(data.value));
        let on_duration = self.link.callback(|data: InputData| EditMeetMessage::Duration(data.value));
        html! { 
            <>
                <div class="d-flex flex-column justify-content-center py-4">
                    <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{lang::dict("Meeting Name")}</span>
                    <input class="input input-style-universal w-100" style="width: 273px;" type="text"
                        placeholder={lang::dict("Class 2")} min="5" max="35" value=self.title.clone() oninput=on_title />
                </div>
                <div class="d-flex flex-column justify-content-center">
                    // <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{lang::dict("Meeting Duration")}</span>
                    <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{"Fecha de la Reunión"}</span>
                    <input class="input input-style-universal w-100 form-date-meet" style="width: 273px;" type="date" min="5" max="35" value=self.schedule_time.clone() oninput=on_duration />
                </div>
            </>
        }
    }
}