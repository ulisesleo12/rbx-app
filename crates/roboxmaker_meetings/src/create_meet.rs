use log::*;
use uuid::Uuid;
use yew::prelude::*;
use crate::ClassGroupMeetings;
use code_location::code_location;
use chrono::{NaiveDate, NaiveTime, Local};
use crate::create_meeting_node::CreateMeetVCalendar;
use crate::button_create_meetings::ButtonCreateMeetings;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, meetings_model};
use roboxmaker_types::types::{SchoolId, GroupId, AppRoute, MeetingsId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct ModalCreateMeet {
    link: ComponentLink<Self>,
    props: ModalCreateMeetProperties,
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    task_add: Option<RequestTask>,
    title: String,
    schedule_time: String,
    start_of_meeting: String,
    end_of_meeting: String,
    meetings: Vec<meetings_model::search_meetings_when_create::SearchMeetingsWhenCreateMeetings>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ModalCreateMeetProperties {
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
pub enum ModalCreateMeetMessage {
    CreateMeetings,
    MeetingsAdded(Option<MeetingsId>),
    Title(String),
    ScheludeTime(String),
    StartOfMeeting(String),
    EndOfMeeting(String),
    FetchSearhMeet(String),
    Meets(Option<meetings_model::search_meetings_when_create::ResponseData>),
}

impl Component for ModalCreateMeet {
    type Message = ModalCreateMeetMessage;
    type Properties = ModalCreateMeetProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ModalCreateMeet {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            task_add: None,
            title: String::from(""),
            schedule_time: String::from(""),
            start_of_meeting: String::from(""),
            end_of_meeting: String::from(""),
            meetings: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ModalCreateMeetMessage::CreateMeetings => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let inventory_group_id = self.props.inventory_group_id;
                    let group_id = self.props.group_id;
                    let name_grade =self.props.class_name.clone();
                    let datetime = self.schedule_time.clone();
                    let start_of_time = self.start_of_meeting.clone();
                    let end_of_time = self.end_of_meeting.clone();
                    
                    let schedule_time = NaiveDate::parse_from_str(&datetime,"%Y-%m-%d").unwrap();
                    let start_of_meeting = NaiveTime::parse_from_str(&start_of_time, "%H:%M").unwrap();            
                    let end_of_meeting = NaiveTime::parse_from_str(&end_of_time, "%H:%M").unwrap();  
                    let local = Local::now().naive_local();
                    
                    let vars = meetings_model::meetings_group_create::Variables {
                        title: self.title.clone(),
                        name_grade: name_grade,
                        content: String::from("Nothing"),
                        group_id: group_id.0,
                        inventory_group_id: inventory_group_id,
                        meet_id: Uuid::new_v4(),
                        schedule_time: schedule_time,
                        start_of_meeting: start_of_meeting,
                        end_of_meeting: end_of_meeting,
                        timestamp: local,
                    };

                    let task = meetings_model::MeetingsGroupCreate::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let meetings_id = response
                                .and_then(|data| data.insert_meetings_content_one)
                                .and_then(|content| Some(MeetingsId(content.meet_id)));
                            ModalCreateMeetMessage::MeetingsAdded(meetings_id)
                        },
                    );
                    self.task_add = Some(task);
                }
            }
            ModalCreateMeetMessage::MeetingsAdded(meetings_id) => {
                if let Some(meetings_id) = meetings_id {
                    self.props.meetings.push(ClassGroupMeetings { meetings_id });
                    if let Some(on_list_change) = &self.props.on_list_change {
                        on_list_change.emit(());
                        self.props.close_modal_callback_meet.emit(false)
                    }
                } else {
                    self.props.close_modal_callback_failed.emit(false);
                }
                self.title = String::from("");
                self.schedule_time = String::from("");
                self.start_of_meeting = String::from("");
                self.end_of_meeting = String::from("");
            }
            ModalCreateMeetMessage::Title(title) => {
                self.title = title;
            }
            ModalCreateMeetMessage::ScheludeTime(schedule_time) => {
                self.schedule_time = schedule_time;
                info!("schedule_time: {:?}", self.schedule_time);
            }
            ModalCreateMeetMessage::StartOfMeeting(start_of_meeting) => {
                self.start_of_meeting = start_of_meeting;
            }
            ModalCreateMeetMessage::EndOfMeeting(end_of_meeting) => {
                self.end_of_meeting = end_of_meeting;
            }
            ModalCreateMeetMessage::FetchSearhMeet(title) => {
                self.title = title;
                if let Some(graphql_task) = self.graphql_task.as_mut() { 
                    let naive = Local::now().date_naive();

                    let datetime = self.schedule_time.clone();

                    let gte = NaiveDate::parse_from_str(&datetime,"%Y-%m-%d").unwrap_or(naive);

                    if self.props.school_id.is_some() {

                        let vars = meetings_model::search_meetings_when_create::Variables {
                            groud_id: self.props.group_id.0.clone(),
                            school_id: self.props.school_id.unwrap().0,
                            title: Some(self.title.clone()),
                            gte: gte,
                            // lte: lte,
                        };
                        let task = meetings_model::SearchMeetingsWhenCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ModalCreateMeetMessage::Meets(response)
                            }
                        );
                        self.task = Some(task);
                    }
                }
            }
            ModalCreateMeetMessage::Meets(response) => {
                self.meetings = response.and_then(|data| Some(data.meetings)).unwrap_or_default();
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
        let on_title = self.link.callback(|data: InputData| ModalCreateMeetMessage::Title(data.value));
        // let on_schedule_time = self.link.callback(|data: InputData| ModalCreateMeetMessage::ScheludeTime(data.value));
        let on_callback_date= self.link.callback(|data| ModalCreateMeetMessage::ScheludeTime(data));
        let on_start_of_meeting = self.link.callback(|data: InputData| ModalCreateMeetMessage::StartOfMeeting(data.value));
        let on_end_of_meeting = self.link.callback(|data: InputData| ModalCreateMeetMessage::EndOfMeeting(data.value));
        let on_select = self.link.callback(|_| ModalCreateMeetMessage::CreateMeetings);
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 3 {
                format!("{}", search.value)
            } else {
                search.value
            };
            ModalCreateMeetMessage::FetchSearhMeet(search)
        });
        let met_invalid = if !self.meetings.is_empty() {
            true
        } else {
            false
        };
        let maybe_meetings_add = {
            let group_id = self.props.group_id;
            if self.props.allow_edit {
                html! {
                    <ButtonCreateMeetings on_select=on_select 
                        allow_create=true
                        group_id=Some(group_id.clone())
                        on_app_route=self.props.on_app_route.clone()
                        title=self.title.clone()
                        schedule_time=self.schedule_time.clone()
                        start_of_meeting=self.start_of_meeting.clone()
                        end_of_meeting=self.end_of_meeting.clone()
                        met_invalid=met_invalid.clone() />
                }
            } else {
                html! {}
            }
        };
        let input_valid_option = if !self.meetings.is_empty() {
            "form-control is-invalid input-style-universal px-3 py-1"
        } else {
            "form-control input-style-universal px-3 py-1"
        };
        let maybe_meet = if !self.meetings.is_empty() {
            html! {
                <div class="invalid-feedback">
                    {lang::dict("Invalid title, meeting already exists")}
                </div>
            }
        } else {
            html! {}
        };
        html! { 
            <>
                <div class="mt-3 w-100">
                    <label class="form-label text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{lang::dict("Meeting Name")}</label>
                    <div class="input-group">
                        <input type="text" class=input_valid_option oninput=on_search.clone() style="width: 330px;" 
                        placeholder={lang::dict("Class 2")} min="5" max="35" value=self.title.clone() oninput=on_title autofocus=true />
                        {maybe_meet}
                    </div>
                </div>
                <div class="mt-3 w-100">
                    <label class="form-label text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{"Fecha de la Reunión"}</label>
                    // <div class="input-group">
                        // <span class="input-group-text text-secondary-purple icon-schedule-time">
                            // <i class="far fa-calendar"></i>
                        // </span>
                        // <input type="date" class="form-control input-style-universal" value=self.schedule_time.clone() oninput=&on_schedule_time />
                    // </div>
                    <CreateMeetVCalendar on_callback_date=on_callback_date />
                </div>
                <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center mt-2">{"Hora de Reunión"}</span>
                <div class="d-flex justify-content-between align-items-center w-100">
                    <div class="d-flex flex-column">
                        <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{"Inicio"}</span>
                        <div class="input-group">
                            <input class="input input-style-universal time-meetings px-3" style="width: 155px;" type="time" value=self.start_of_meeting.clone() oninput=on_start_of_meeting />
                        </div>
                    </div>
                    <div class="d-flex flex-column">
                        <span class="text-purple-gray noir-bold is-size-16 lh-20 mb-1 text-center">{"Final"}</span>
                        <div class="input-group">
                            <input class="input input-style-universal time-meetings-2 px-3" style="width: 155px;" type="time" value=self.end_of_meeting.clone() oninput=on_end_of_meeting />
                        </div>
                    </div>
                </div>
                <div class="d-flex justify-content-center pt-4">{maybe_meetings_add} </div>
            </>
        }
    }
}