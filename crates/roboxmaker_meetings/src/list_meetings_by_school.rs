use log::*;
use chrono::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::meetings_model;
use roboxmaker_types::types::{AppRoute, SchoolId, MeetingsId, GroupId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};


#[derive(Debug, Clone, PartialEq)]
pub struct MeetingsProfile {
    pub meetings_id: MeetingsId,
    pub group_id: GroupId,
    pub meet_title: String,
    pub name_grade: String,
    pub schedule_time: String, 
    pub filter_schedule_time: String, 
    pub end_of_meeting: Option<NaiveTime>, 
    pub author_name: String,
    pub user_staff: bool,
    pub user_teacher: bool,
}

pub struct MeetingsListBySchool {
    graphql_task: Option<GraphQLTask>,
    meetings_sub: Option<SubscriptionTask>,
    meeting_delete: Option<RequestTask>,
    meetings: Vec<MeetingsProfile>,
    display_list_meetings: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MeetingsListBySchoolProperties {
    pub school_id: SchoolId,
    pub school_name: String,
    pub date_selected: String,
}

#[derive(Debug)]
pub enum MeetingsListBySchoolMessage {
    FetchMeetingsStarted,
    MeetingsStarted(Option<meetings_model::list_scheduled_meetings_by_school_id::ResponseData>),
    DeletedMeet(MeetingsId),
    MeetDeleted(Option<meetings_model::delete_meet_by_id::ResponseData>),
    SchowMeetingsBySchool,
}

impl Component for MeetingsListBySchool {
    type Message = MeetingsListBySchoolMessage;
    type Properties = MeetingsListBySchoolProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MeetingsListBySchoolMessage::FetchMeetingsStarted);
        
        MeetingsListBySchool {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            meetings_sub: None,
            meeting_delete: None,
            meetings: vec![],
            display_list_meetings: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MeetingsListBySchoolMessage::FetchMeetingsStarted => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let datetime = ctx.props().date_selected.clone();
                    let scheduled_meetings = NaiveDate::parse_from_str(&datetime,"%Y-%m-%d").unwrap();

                    let vars = meetings_model::list_scheduled_meetings_by_school_id::Variables { 
                        school_id: ctx.props().school_id.0,
                        scheduled_meetings: scheduled_meetings,
                    };

                    let task = meetings_model::ListScheduledMeetingsBySchoolId::subscribe(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            MeetingsListBySchoolMessage::MeetingsStarted(response)
                        },
                    );
                    self.meetings_sub = Some(task);
                }
            }
            MeetingsListBySchoolMessage::MeetingsStarted(meetings_response) => {
                self.meetings = meetings_response.and_then(|data| Some(data.meetings_profile))
                    .unwrap_or(vec![])
                    .iter().map(|meetings| {
                    let group_id = meetings.meetings_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
                    let author_name = meetings.author_profile.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                    let user_staff = meetings.author_profile.clone().and_then(|data| data.user_staff).is_some();
                    let user_teacher = meetings.author_profile.clone().and_then(|data| data.user_teacher).is_some();
                    MeetingsProfile {
                        meetings_id: MeetingsId(meetings.meet_id),
                        group_id: GroupId(group_id),
                        meet_title: meetings.title.clone(),
                        name_grade: meetings.name_grade.clone(),
                        schedule_time: meetings.schedule_time.format("%d-%m-%Y").to_string(),
                        filter_schedule_time: meetings.schedule_time.format("%Y-%m-%d").to_string(),
                        end_of_meeting: meetings.end_of_meeting,
                        author_name: author_name,
                        user_staff: user_staff,
                        user_teacher: user_teacher,
                    }
                }).collect()
            }
            MeetingsListBySchoolMessage::DeletedMeet(meetings_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::delete_meet_by_id::Variables {
                        meetings_id: meetings_id.0
                    };
                    let task = meetings_model::DeleteMeetById::request(
                        graphql_task,
                        &ctx,
                            vars,
                            |response| {
                                MeetingsListBySchoolMessage::MeetDeleted(response)
                            },
                    );
                    self.meeting_delete = Some(task);
                }
            }
            MeetingsListBySchoolMessage::MeetDeleted(_response) => {
            }
            MeetingsListBySchoolMessage::SchowMeetingsBySchool => {
                self.display_list_meetings = !self.display_list_meetings;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if  ctx.props().date_selected != old_props.date_selected {
            ctx.link().send_message(MeetingsListBySchoolMessage::FetchMeetingsStarted);
        }
        
        if ctx.props() != old_props {
            should_render = true;
        } 

        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let list_meetings_start = self
            .meetings
            .iter().map(|meetings_profile| {
                let group_id  = meetings_profile.group_id;
                let meetings_id = meetings_profile.meetings_id;

                let navigator = ctx.link().navigator().unwrap();
                let on_meet = Callback::from(move |_| navigator.push(&AppRoute::Meet{group_id, meetings_id}));
                let on_delete_meet = ctx.link().callback(move |_| MeetingsListBySchoolMessage::DeletedMeet(meetings_id));
                let maybe_title = if meetings_profile.meet_title.len() == 0 {
                    html! {
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 order-0 mb-sm-2 mb-md-2 mb-lg-0 col-sm-8 col-md-8 col-lg-3">{"Reunión sin Titulo"}</span>
                    }
                } else {
                    html! {
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 order-0 mb-sm-2 mb-md-2 mb-lg-0 col-sm-8 col-md-8 col-lg-3">{&meetings_profile.meet_title}</span>
                    }
                };
                let naivedate_local = Local::now().date_naive();
                let naivetime_local = Local::now().time();
                let maybe_status = if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) < meetings_profile.end_of_meeting {
                    html! {
                        <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("In progress")}</span>
                    }
                } else if naivedate_local.format("%d-%m-%Y").to_string() < meetings_profile.schedule_time {
                    html! {
                        <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("Scheduled")}</span>
                    }
                } else if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) > meetings_profile.end_of_meeting {
                    html! {
                        <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("Meeting Ended")}</span>
                    }
                } else {
                    html! {}
                };
                let maybe_disabled = if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) > meetings_profile.end_of_meeting {
                    true
                } else if naivedate_local.format("%d-%m-%Y").to_string() > meetings_profile.schedule_time {
                    true
                } else {
                    false
                };
                let maybe_disabled_delete = if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) > meetings_profile.end_of_meeting {
                    "btn btn-danger btn-disabled-delete-meet"
                } else if naivedate_local.format("%d-%m-%Y").to_string() > meetings_profile.schedule_time {
                    "btn btn-danger btn-disabled-delete-meet"
                } else {
                    "btn btn-outline-danger"
                };
                let maybe_status_btn = if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) < meetings_profile.end_of_meeting ||
                    naivedate_local.format("%d-%m-%Y").to_string() < meetings_profile.schedule_time {
                    html! {
                        <a class="btn button-meet-2 me-5" onclick={on_meet}>
                            <span class="text-white noir-bold is-size-18 lh-22">{lang::dict("Meet")}</span>
                        </a>
                    }
                } else if naivedate_local.format("%d-%m-%Y").to_string() == meetings_profile.schedule_time && Some(naivetime_local) > meetings_profile.end_of_meeting {
                    html! {
                        <a class="btn btn-meet-2-disabled me-5">
                            <span class="text-white noir-bold is-size-18 lh-22">{lang::dict("Meet")}</span>
                        </a>
                    }
                } else {
                    html! {}
                };
                let maybe_rol = if meetings_profile.user_staff {
                    html! {
                        <span>{lang::dict("Staff")}</span>
                    }
                } else {
                    html! {
                        <span>{lang::dict("Teacher")}</span>
                    }
                };
                let maybe_list_meets = if meetings_profile.filter_schedule_time == ctx.props().date_selected {
                    html! {
                        <div class="d-flex flex-wrap justify-content-between align-items-center w-100 my-3">
                            {maybe_title}
                            <div class="d-flex flex-wrap justify-content-between order-sm-2 order-md-2 order-lg-1 col-sm-12 col-md-12 col-lg-7">
                                <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{&meetings_profile.name_grade}</span>
                                {maybe_status}
                                <div class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">
                                    <span>{&meetings_profile.author_name}</span>
                                    <span class="mx-2">{"-"}</span>
                                    {maybe_rol}
                                </div>
                                <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17">
                                    <i class="far fa-clock me-1"></i>
                                    <span>{&meetings_profile.schedule_time}</span>
                                </span>
                            </div>
                            <div class="order-sm-1 order-md-1 order-lg-2 mb-sm-2 mb-md-2 mb-lg-0">
                                {maybe_status_btn}
                                <button class={maybe_disabled_delete} onclick={on_delete_meet} disabled={maybe_disabled}>
                                    <span class="is-size-20">
                                        <i class="far fa-trash-alt"></i>
                                    </span>
                                </button>
                            </div>
                        </div>
                    }
                } else {
                    html! {}  
                };
                html! {
                    {maybe_list_meets}
                }
            }).collect::<Html>();
            
        let on_show_meetings = ctx.link().callback(move |_| MeetingsListBySchoolMessage::SchowMeetingsBySchool);

        let button_show_meetings = if self.display_list_meetings {
            html! {
                <a onclick={&on_show_meetings} class="text-gray-blue">
                    <i class="fas fa-angle-up"></i>
                </a>
            }
        } else {
            html! {
                <a onclick={&on_show_meetings} class="text-gray-blue">
                    <i class="fas fa-angle-down"></i>
                </a>
            }
        };
        let class_meetings = if self.display_list_meetings {
            "meetings-container-card-true d-flex align-items-center justify-content-between mb-2 px-5"
        } else {
            "meetings-container-card-false d-flex align-items-center justify-content-between px-5"
        };
        let class_meetings_child = if self.display_list_meetings {
            "meetings-container-child-true p-5"
        } else {
            "d-none"
        };
        let maybe_meets = if !self.meetings.is_empty() {
            html! {
                {list_meetings_start}
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="is-size-7-mobile is-size-5-tablet is-size-4-desktop">{lang::dict("No meetigns here.")}</span>
                </div>
            }
        };
        html! {
            <div class="pb-5">
                <div class={class_meetings}>
                    <div class="d-flex flex-wrap">
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{&ctx.props().school_name}</span>
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 ps-2">{"("}{&self.meetings.len()}{")"}</span>
                    </div>
                    {button_show_meetings}
                </div>
                <div class={class_meetings_child}>
                    {maybe_meets}
                </div>
            </div>
        }
    }
}