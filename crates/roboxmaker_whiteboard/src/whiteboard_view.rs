use log::*;
use yew::prelude::*;
use yew::{html, Component, Html};

use roboxmaker_main::config;
use roboxmaker_types::types::{WhiteboardId, MyUserProfile};

pub struct WhiteboardPage {
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct WhiteboardPageProperties {
    pub user_profile: Option<MyUserProfile>,
    pub whiteboard_id: WhiteboardId,
}

#[derive(Debug)]
pub enum WhiteboardPageMessage {}

impl Component for WhiteboardPage {
    type Message = WhiteboardPageMessage;
    type Properties = WhiteboardPageProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        WhiteboardPage {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        info!("{:?} ", ctx.props());
        // let mut should_render = false;
        // if self.props != props {
        //     self.props = props;
        //     should_render = true;
        // }
        // should_render
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let maybe_whiteboard = if let Some(_auth_user) = &ctx.props().user_profile {
            // let _display_name = auth_user.user_by_pk.clone().and_then(|data| data.user_profile)
            //     .and_then(|user_profile| Some(user_profile.full_name).clone())
            //     .unwrap_or(String::from(lang::dict("Anonymous Guest")));
            let whiteboard_url = format!(
                "{}/boards/{}",
                config::AKER_WBO_URL,
                ctx.props().whiteboard_id.0
            );
            html! {
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <iframe allow="camera; microphone; fullscreen; display-capture" src={whiteboard_url}
                                style="min-height: 700px; width: 100%; border: 0px; padding: 0px, margin: 0px;"></iframe>
                        </div>
                    </div>
                </section>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                <div>
                    {maybe_whiteboard}
                </div>
            </div>
        }
    }
}
