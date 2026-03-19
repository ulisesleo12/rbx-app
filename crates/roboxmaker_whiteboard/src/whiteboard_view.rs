use log::*;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::config;
use roboxmaker_types::types::{WhiteboardId, MyUserProfile};

pub struct WhiteboardPage {
    _link: ComponentLink<Self>,
    props: WhiteboardPageProperties,
    _task: Option<FetchTask>,
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

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        WhiteboardPage {
            _link,
            props,
            _task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        true
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
        let maybe_whiteboard = if let Some(_auth_user) = &self.props.user_profile {
            // let _display_name = auth_user.user_by_pk.clone().and_then(|data| data.user_profile)
            //     .and_then(|user_profile| Some(user_profile.full_name).clone())
            //     .unwrap_or(String::from(lang::dict("Anonymous Guest")));
            let whiteboard_url = format!(
                "{}/boards/{}",
                config::AKER_WBO_URL,
                self.props.whiteboard_id.0
            );
            html! {
                <section class="hero">
                    <div class="hero-body">
                        <div class="container">
                            <iframe allow="camera; microphone; fullscreen; display-capture" src=whiteboard_url
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
