use log::*;
use yew::prelude::*;
use yew::{html, Component, Html};


pub struct ImgView {
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

    fn create(_ctx: &Context<Self>) -> Self {
        ImgView { }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        match msg {}
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
            <img class="img-card-72" src={ctx.props().pic_path.clone()} alt="" />
        }
    }
}
