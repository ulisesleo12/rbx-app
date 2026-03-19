use log::*;
use yew::prelude::*;
use code_location::code_location;
use crate::{meet_session::MeetSession};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::{lang, config};
use roboxmaker_models::meetings_model;
use roboxmaker_types::types::{GroupId, MeetingsId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct MeetPage {
    link: ComponentLink<Self>,
    props: MeetPageProperties,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(MeetPageMessage::FetchDataMeet);
        MeetPage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            meet_title_task: None,
            whiteboard_on: false,
            class_name: vec![],
            meet_title: vec![],    
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MeetPageMessage::FetchDataMeet => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::class_name_and_meet_title::Variables { 
                        group_id: self.props.group_id.0,
                        meeting_id: self.props.meetings_id.0,
                    };

                    let task = meetings_model::ClassNameAndMeetTitle::request(
                        graphql_task,
                        &self.link,
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let should_render = false;
        if self.props != props {
            self.props = props;
        }
        should_render
    }

    fn view(&self) -> Html {
        let on_toggle_whiteboard = self
            .link
            .callback(move |_| MeetPageMessage::ToggleWhiteboard);
        let data_meet = self.class_name.iter().zip(self.meet_title.clone()).map(|(class, meet)| {
            html! {
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {&class.name}{" - "}{&meet.title}
                </h1>
            }
        }).collect::<Html>();
        let maybe_whiteboard = self.props.user_profile.as_ref()
            .and_then(|item| {
                let _display_name = item.full_name.clone();
                let whiteboard_url = format!(
                    "{}/boards/{}",
                    config::AKER_WBO_URL,
                    self.props.meetings_id.0
                );
                let iframe = html!{
                    <iframe allow="camera; microphone; fullscreen; display-capture" src=whiteboard_url
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
                            <div class=maybe_class>
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
                <a onclick=&on_toggle_whiteboard class="btn btn-outline-primary-blue-dark px-5 col-2">
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
                        <MeetSession user_profile=self.props.user_profile.clone()
                            domain=domain
                            group_id=self.props.group_id
                            meetings_id=self.props.meetings_id />
                    </div>
                </div>
            </>
        }
    }
}
