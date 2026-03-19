use log::*;
use chrono::Local;
use yew::prelude::*;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_types::types::{GroupId, AppRoute, MeetingsProfile};
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

pub struct MeetingsListHome {
    meetings_list: Vec<MeetingsProfile>,
    list_meetings_state: LoadMeetings,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MeetingsListHomeProps {
    pub group_id: GroupId,
    pub meetings_list: Vec<MeetingsProfile>,
}

#[derive(Debug)]
pub enum MeetingsListHomeMessage {
    FetchMeetingsByGroupId,
}

impl Component for MeetingsListHome {
    type Message = MeetingsListHomeMessage;
    type Properties = MeetingsListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MeetingsListHomeMessage::FetchMeetingsByGroupId);
        
        MeetingsListHome {
            meetings_list: vec![],
            list_meetings_state: LoadMeetings::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            MeetingsListHomeMessage::FetchMeetingsByGroupId => {
                self.list_meetings_state = LoadMeetings::Loading;

                self.meetings_list = ctx.props().meetings_list.clone();

                if !self.meetings_list.is_empty() {
                    self.list_meetings_state = LoadMeetings::Load(LoadMeetingsFound::Found);
                } else {
                    self.list_meetings_state = LoadMeetings::Load(LoadMeetingsFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().meetings_list != old_props.meetings_list {
            ctx.link().send_message(MeetingsListHomeMessage::FetchMeetingsByGroupId);
        }

        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id  = ctx.props().group_id;
        let card_meetings_list = self.meetings_list.iter().map(|item| {
            let meetings_id = item.meeting_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_meet = Callback::from(move |_| navigator.push(&AppRoute::Meet{group_id, meetings_id}));

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
                    <a class="btn button-meet order-sm-1 order-md-1 order-lg-2 mb-sm-2 mb-md-2 mb-lg-0" onclick={&on_meet}>
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