use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use crate::direct_meet_session::DirectMeetSession;

use roboxmaker_models::grade_model;
use roboxmaker_main::{lang, config};
use roboxmaker_types::types::{GroupId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct DirectMeetingRoom {
    graphql_task: Option<GraphQLTask>,
    direct_meet_task: Option<RequestTask>,
    whiteboard_on: bool,
    class_name: Vec<grade_model::name_of_degree_by_id::NameOfDegreeByIdClassProfile>,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct DirectMeetingRoomProperties {
    pub user_profile: Option<MyUserProfile>,
    pub group_id: GroupId,
}

#[derive(Debug)]
pub enum DirectMeetingRoomMessage {
    FetchClassName,
    ClassName(Option<grade_model::name_of_degree_by_id::ResponseData>),
    ToggleWhiteboard,
}

impl Component for DirectMeetingRoom {
    type Message = DirectMeetingRoomMessage;
    type Properties = DirectMeetingRoomProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(DirectMeetingRoomMessage::FetchClassName);

        roboxmaker_utils::functions::school_state();

        DirectMeetingRoom {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            direct_meet_task: None,
            whiteboard_on: false,
            class_name: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            DirectMeetingRoomMessage::FetchClassName => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = grade_model::name_of_degree_by_id::Variables {
                        group_id: ctx.props().group_id.0, 
                    };
                    let task = grade_model::NameOfDegreeById::request(
                        graphql_task, 
                        &ctx, 
                        vars, 
                        |response| {
                            DirectMeetingRoomMessage::ClassName(response)
                        }
                    );
                    self.direct_meet_task = Some(task);
                }
            }
            DirectMeetingRoomMessage::ClassName(class_name) => {
                self.class_name = class_name.clone().and_then(|data| Some(data.class_profile)).unwrap_or(vec![])
            }
            DirectMeetingRoomMessage::ToggleWhiteboard => self.whiteboard_on = !self.whiteboard_on,
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
            .callback(move |_| DirectMeetingRoomMessage::ToggleWhiteboard);
        let class_name = self.class_name.iter().map(|item| {
            html! {
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {&item.name}
                </h1>
            }
        }).collect::<Html>();
        let maybe_whiteboard = ctx.props().user_profile.as_ref()
            .and_then(|item| {
                let _display_name = item.full_name.clone();
                let whiteboard_url = format!(
                    "{}/boards/{}",
                    config::AKER_WBO_URL,
                    ctx.props().group_id.0
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
                }
                else {
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
                    {class_name}
                    {toggle}
                    <div class="d-flex flex-wrap">
                        {maybe_whiteboard.clone()}
                        <DirectMeetSession user_profile={ctx.props().user_profile.clone()}
                            domain={domain}
                            group_id={ctx.props().group_id} />
                    </div>
                </div>
            </>
        }
    }
}