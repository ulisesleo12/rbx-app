use log::*;
use chrono::Local;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::meetings_model;
use roboxmaker_types::types::{AppRoute, SchoolId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

#[derive(Debug, Clone, PartialEq)]
pub struct MeetingsProfileFinished {
    pub schedule_time: String, 
    pub timestamp: String,
}

pub struct LastMeetingsList {
    link: ComponentLink<Self>,
    props: LastMeetingsListProperties,
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    meetings_finished: Vec<MeetingsProfileFinished>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LastMeetingsListProperties {
    pub on_app_route: Callback<AppRoute>,
    pub school_id: SchoolId,
    pub school_name: String,
}

#[derive(Debug)]
pub enum LastMeetingsListMessage {
    FetchMeetingsFinished,
    MeetingsFinished(Option<meetings_model::list_last_meetings_by_school_id::ResponseData>),
}

impl Component for LastMeetingsList {
    type Message = LastMeetingsListMessage;
    type Properties = LastMeetingsListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(LastMeetingsListMessage::FetchMeetingsFinished);
        LastMeetingsList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            meetings_finished: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            LastMeetingsListMessage::FetchMeetingsFinished => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let last_meetings = Local::now().date_naive();
                    let vars = meetings_model::list_last_meetings_by_school_id::Variables { 
                        school_id: self.props.school_id.0,
                        last_meetings: last_meetings,
                        // end_of_meeting: end_of_meeting,
                    };
                    let task = meetings_model::ListLastMeetingsBySchoolId::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            LastMeetingsListMessage::MeetingsFinished(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            LastMeetingsListMessage::MeetingsFinished(response) => {
                self.meetings_finished = response.and_then(|data| Some(data.meetings_profile))
                    .unwrap_or_default()
                    .iter()
                    .map(|meetings| {
                        MeetingsProfileFinished {
                            schedule_time: meetings.schedule_time.format("%d-%m-%Y").to_string(),
                            timestamp: meetings.timestamp.format("%e/%b/%Y").to_string(),
                        }
                    }).collect();
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let should_render = false;
        if self.props != props {
            self.props = props;
            true;
        } else {
            false;
        }
        should_render
    }

    fn view(&self) -> Html {
        let list_meetings_finished = self
            .meetings_finished
            .iter().map(|meetings_profile| {
                html! {
                    <div class="card-last-meettings bg-white d-flex flex-column justify-content-center p-4 mb-4">
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{self.props.school_name.clone()}</span>
                        <div class="d-flex align-items-center justify-content-between pt-3">
                            <span class="text-gray-blue noir-regular is-size-14 lh-18 d-flex align-items-center">
                                <i class="far fa-calendar me-1"></i>
                                <span>{&meetings_profile.timestamp}</span>
                            </span>
                            <span class="text-gray-blue noir-regular is-size-14 lh-18 d-flex align-items-center">
                                <i class="far fa-clock me-1"></i>
                                <span>{&meetings_profile.schedule_time}</span>
                            </span>
                        </div>
                    </div>
                }
            }).collect::<Html>();
        html! {
            {list_meetings_finished}
        }
    }
}