use log::*;
use chrono::Local;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, meetings_model};
use roboxmaker_types::types::{GroupId, AppRoute, MeetingsId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_loaders::placeholders::card_meetings_home::CardMeetingssHomePlaceholder;

#[derive(Debug, Clone)]
enum LoadMeetingsFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadMeetings {
    Loading,
    Load(LoadMeetingsFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeetingsProfile {
    pub title: String,
    pub schedule_time: String,
    pub start_of_meeting: String,
    pub end_of_meeting: String,
    pub meeting_id: MeetingsId,
    pub author_name: String,
    pub user_staff: bool,
    pub user_teacher: bool,
}

pub struct MeetingsListHome {
    link: ComponentLink<Self>,
    props: MeetingsListHomeProps,
    graphql_task: Option<GraphQLTask>,
    meetings_sub: Option<SubscriptionTask>,
    meetings_list: Vec<MeetingsProfile>,
    list_meetings_state: LoadMeetings,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MeetingsListHomeProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum MeetingsListHomeMessage {
    AppRoute(AppRoute),
    FetchMeetingsByGroupId,
    Meetings(Option<meetings_model::meetings_by_group_id::ResponseData>),
}

impl Component for MeetingsListHome {
    type Message = MeetingsListHomeMessage;
    type Properties = MeetingsListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(MeetingsListHomeMessage::FetchMeetingsByGroupId);
        MeetingsListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            meetings_sub: None,
            meetings_list: vec![],
            list_meetings_state: LoadMeetings::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            MeetingsListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            MeetingsListHomeMessage::FetchMeetingsByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    self.list_meetings_state = LoadMeetings::Loading;
                    let group_id = self.props.group_id;
                    let scheduled_meetings = Local::now().date_naive();

                    let vars = meetings_model::meetings_by_group_id::Variables {
                        group_id: group_id.0,
                        limit: 10,
                        scheduled_meetings,
                    };

                    let task = meetings_model::MeetingsByGroupId::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                MeetingsListHomeMessage::Meetings(response)
                            },
                    );
                    self.meetings_sub = Some(task);
                }
            }
            MeetingsListHomeMessage::Meetings(response) => {
                self.meetings_list = response
                    .clone()
                    .and_then(|data| Some(data.meetings_profile))
                    .unwrap_or_default()
                    .iter()
                    .map(|meetings_profile| {
                        let title = meetings_profile.title.clone();
                        let meeting_id = meetings_profile.meet_id.clone();
                        let author_name = meetings_profile.author_profile.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                        let user_staff = meetings_profile.author_profile.clone().and_then(|data| data.user_staff).is_some();
                        let user_teacher = meetings_profile.author_profile.clone().and_then(|data| data.user_teacher).is_some();
                        MeetingsProfile {
                            title: title,
                            schedule_time: meetings_profile.schedule_time.format("%d-%m-%Y").to_string(),
                            start_of_meeting: meetings_profile.start_of_meeting.unwrap().to_string(),
                            end_of_meeting: meetings_profile.end_of_meeting.unwrap().to_string(),
                            meeting_id: MeetingsId(meeting_id),
                            author_name: author_name,
                            user_staff: user_staff,
                            user_teacher: user_teacher,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.meetings_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_meetings_state = LoadMeetings::Load(LoadMeetingsFound::Found);
                } else {
                    self.list_meetings_state = LoadMeetings::Load(LoadMeetingsFound::NotFound);
                }
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut schould_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(MeetingsListHomeMessage::FetchMeetingsByGroupId);
        }

        if self.props != props {
            self.props = props;
            schould_render = true;
        }
        
        schould_render
    }

    fn view(&self) -> Html {
        let group_id  = self.props.group_id;
        let card_meetings_list = self.meetings_list.iter().map(|item| {
            let meeting_id = item.meeting_id;
            let on_meet = self.link.callback(move |_| MeetingsListHomeMessage::AppRoute(AppRoute::Meet(group_id, meeting_id)));
            let maybe_title = if item.title.len() == 0 {
                html! {
                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 order-0 mb-sm-2 mb-md-2 mb-lg-0 col-sm-8 col-md-8 col-lg-3">{"Reunión sin Titulo"}</span>
                }
            } else {
                html! {
                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 order-0 mb-sm-2 mb-md-2 mb-lg-0 col-sm-8 col-md-8 col-lg-3">{&item.title}</span>
                }
            };
            let naivedate_local = Local::now().date_naive();
            let naivetime_local = Local::now().time();
            let _maybe_date_meet = if naivedate_local.format("%d-%m-%Y").to_string() == item.schedule_time {
                html! {
                    <span class="me-1">{"Hoy"}</span>
                }
            } else {
                html! {
                    <span class="me-1">{&item.schedule_time}</span>
                }
            };
            let maybe_status = if naivedate_local.format("%d-%m-%Y").to_string() == item.schedule_time && naivetime_local.format("%H:%M:%S").to_string() < item.end_of_meeting {
                html! {
                    <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("In progress")}</span>
                }
            } else if naivedate_local.format("%d-%m-%Y").to_string() < item.schedule_time {
                html! {
                    <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("Scheduled")}</span>
                }
            } else if naivedate_local.format("%d-%m-%Y").to_string() == item.schedule_time && naivetime_local.format("%H:%M:%S").to_string() > item.end_of_meeting {
                html! {
                    <span class="text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">{lang::dict("Meeting Ended")}</span>
                }
            } else {
                html! {}
            };
            let maybe_status_btn = if naivedate_local.format("%d-%m-%Y").to_string() == item.schedule_time && naivetime_local.format("%H:%M:%S").to_string() < item.end_of_meeting ||
                naivedate_local.format("%d-%m-%Y").to_string() < item.schedule_time {
                html! {
                    <a class="btn button-meet order-sm-1 order-md-1 order-lg-2 mb-sm-2 mb-md-2 mb-lg-0" onclick=&on_meet>
                        {lang::dict("Meet")}
                    </a>
                }
            } else if naivedate_local.format("%d-%m-%Y").to_string() == item.schedule_time && naivetime_local.format("%H:%M:%S").to_string() > item.end_of_meeting {
                html! {
                    <a class="btn btn-disabled me-5 order-sm-1 order-md-1 order-lg-2 mb-sm-2 mb-md-2 mb-lg-0">
                        {lang::dict("Meet")}
                    </a>
                }
            } else {
                html! {}
            };
            let maybe_rol = if item.user_staff {
                html! {
                    <span>{lang::dict("Staff")}</span>
                }
            } else {
                html! {
                    <span>{lang::dict("Teacher")}</span>
                }
            };
            html! {
                <div class="card-meet-class bg-white d-flex flex-wrap justify-content-between align-items-center mb-4 p-4">
                    {maybe_title}
                    <div class="d-flex flex-wrap justify-content-between order-sm-2 order-md-2 order-lg-1 col-sm-12 col-md-12 col-lg-7">
                        <div class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 me-md-5 me-lg-2">
                            <span>{&item.author_name}</span>
                            <span class="mx-2">{"-"}</span>
                            {maybe_rol}
                        </div>
                        {maybe_status}
                        <span class="text-purple-gray noir-light is-size-14 lh-17">
                            <span class="d-flex flex-row align-items-center">
                                <i class="far fa-clock me-1"></i>
                                <span class="me-1">{&item.schedule_time}</span>
                                <span>{"De "}{&item.start_of_meeting}</span>
                                <span class="ms-1">{"- Hasta "}{&item.end_of_meeting}</span>
                            </span>
                        </span>
                    </div>
                    {maybe_status_btn}
                </div>
            }
        }).collect::<Html>();

        let meetings_list = match self.list_meetings_state {
            LoadMeetings::Loading => {
                html! {
                    <CardMeetingssHomePlaceholder />
                }
            },
            LoadMeetings::Load(LoadMeetingsFound::Found) => {
                html! {
                    <> 
                        {card_meetings_list}
                    </>
                }
            },
            LoadMeetings::Load(LoadMeetingsFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No meetigns here.")}</p>
                    </div>
                }
            },
        };

        html! {
            <>
                {meetings_list}
            </>
        }
    }
}