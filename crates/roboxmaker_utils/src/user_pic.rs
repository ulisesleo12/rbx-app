use log::*;
use yew::prelude::*;
use roboxmaker_types::types::MyUserProfile;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct UserPic {
    props: Props,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum Message {}

impl Component for UserPic {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {

        UserPic {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        let user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72 ms-4" src=pic_path.clone() alt="Photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });

        html! {
            { user_profile_pic }
        }
    }
}