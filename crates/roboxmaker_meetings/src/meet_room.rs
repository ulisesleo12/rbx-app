use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use crate::{meet_session::MeetSession};

use roboxmaker_main::{lang, config};
use roboxmaker_models::meetings_model;
use roboxmaker_types::types::{GroupId, MeetingsId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct MeetPage {
    graphql_task: Option<GraphQLTask>,
    meet_title_task: Option<RequestTask>,
    whiteboard_on: bool,
    class_name: Vec<meetings_model::class_name_and_meet_title::ClassNameAndMeetTitleClassProfile>, 
    meet_title: Vec<meetings_model::class_name_and_meet_title::ClassNameAndMeetTitleMeetingsProfile>,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct MeetPageProperties {
    pub user_profile: Option<MyUserProfile>,
    pub group_id: GroupId,
    pub meetings_id: MeetingsId,
}

#[derive(Debug)]
pub enum MeetPageMessage {
    FetchDataMeet,
    DataMeet(Option<meetings_model::class_name_and_meet_title::ResponseData>),
    ToggleWhiteboard,
}

impl Component for MeetPage {
    type Message = MeetPageMessage;
    type Properties = MeetPageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MeetPageMessage::FetchDataMeet);

        roboxmaker_utils::functions::school_state();
        
        MeetPage {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            meet_title_task: None,
            whiteboard_on: false,
            class_name: vec![],
            meet_title: vec![],    
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MeetPageMessage::FetchDataMeet => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::class_name_and_meet_title::Variables { 
                        group_id: ctx.props().group_id.0,
                        meeting_id: ctx.props().meetings_id.0,
                    };

                    let task = meetings_model::ClassNameAndMeetTitle::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            MeetPageMessage::DataMeet(response)
                        },
                    );
                    self.meet_title_task = Some(task);
                }
            }
            MeetPageMessage::DataMeet(data) => {
                self.class_name = data
                    .clone()
                    .and_then(|data| Some(data.class_profile))
                    .unwrap_or(vec![]);

                self.meet_title = data
                    .clone()
                    .and_then(|data| Some(data.meetings_profile))
                    .unwrap_or(vec![]);
            }
            MeetPageMessage::ToggleWhiteboard => {
                self.whiteboard_on = !self.whiteboard_on;
            }
        }
        should_update
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
        let on_toggle_whiteboard = ctx
            .link()
            .callback(move |_| MeetPageMessage::ToggleWhiteboard);
        let data_meet = self.class_name.iter().zip(self.meet_title.clone()).map(|(class, meet)| {
            html! {
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {&class.name}{" - "}{&meet.title}
                </h1>
            }
        }).collect::<Html>();
        let maybe_whiteboard = ctx.props().user_profile.as_ref()
            .and_then(|item| {
                let _display_name = item.full_name.clone();
                let whiteboard_url = format!(
                    "{}/boards/{}",
                    config::AKER_WBO_URL,
                    ctx.props().meetings_id.0
                );
                let iframe = html!{
                    <iframe allow="camera; microphone; fullscreen; display-capture" src={whiteboard_url}
                        style="min-height: 700px; width: 100%; border: 0px; padding: 0px, margin: 0px;"></iframe>
                };

                let maybe_class = if self.whiteboard_on {
                    "col-sm-12 col-md-12 col-lg-6"
                } else {
                    ""
                };
                if self.whiteboard_on {
                    Some(html! {
                        <>
                            <div class={maybe_class}>
                                {iframe}
                            </div>
                        </>
                    })
                } else {
                    Some(html! {
                        <div>
                        </div>
                    })
                }
            })
            .unwrap_or_default();
        let toggle = html!{
            <div class="mt-5 mb-3">
                <a onclick={&on_toggle_whiteboard} class="btn btn-outline-primary-blue-dark px-5 col-2">
                    <i class="fas fa-chalkboard me-3"></i>
                    <span>{lang::dict("Whiteboard")}</span>
                </a>
            </div>
        };

        let domain = config::AKER_MEET_URL.strip_prefix("https://").unwrap();
        html! {
            <>
                <div class="d-flex flex-column scroll-y w-100 p-3 p-md-5 p-lg-7">
                    {data_meet}
                    {toggle}
                    <div class="d-flex flex-wrap">
                        {maybe_whiteboard}
                        <MeetSession user_profile={ctx.props().user_profile.clone()}
                            domain={domain}
                            group_id={ctx.props().group_id}
                            meetings_id={ctx.props().meetings_id} />
                    </div>
                </div>
            </>
        }
    }
}
