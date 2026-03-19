use log::*;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct ImgView {
    _link: ComponentLink<Self>,
    props: ImgViewProperties,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ImgViewProperties {
    pub pic_path: String,
}

#[derive(Debug)]
pub enum ImgViewMessage {}

impl Component for ImgView {
    type Message = ImgViewMessage;
    type Properties = ImgViewProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ImgView {
            _link,
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        match msg {}
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
        // let maybe_user_profile_pic = self
        //     .props
        //     .auth_user
        //     .as_ref()
        //     .and_then(|data| data.user_by_pk.as_ref())
        //     .and_then(|user| user.user_profile.as_ref())
        //     .and_then(|user_profile| user_profile.pic_path.as_ref())
        //     .and_then(|pic_path: &String| {
        //         Some(html! {
        //             <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
        //         })
        //     })
        //     .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
        //     });

        html! { 
            <img class="img-card-72" src={self.props.pic_path.clone()} alt="" />
        }
    }
}
