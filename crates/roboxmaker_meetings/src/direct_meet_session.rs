use log::*;
use web_sys::Node;
use wasm_bindgen::prelude::*;
use yew::{html, Component, Html};
use yew::{virtual_dom::VNode, Context, Properties};

use roboxmaker_types::types::{MyUserProfile, GroupId};

pub struct DirectMeetSession {
    node: Node,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct DirectMeetSessionProperties {
    pub user_profile: Option<MyUserProfile>,
    pub domain: String,
    pub group_id: GroupId,
}

#[derive(Debug)]
pub enum DirectMeetSessionMessage {}

impl Component for DirectMeetSession {
    type Message = DirectMeetSessionMessage;
    type Properties = DirectMeetSessionProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        let node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));
        
            DirectMeetSession {
            node: node.unwrap(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        true
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
        let is_student = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| item.user_student.as_ref())
            .is_some();
        let maybe_meet = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                let meet_room = ctx.props().group_id.0.to_string();
                let meet_user = item.full_name.clone();
                render_meet(
                    &self.node,
                    ctx.props().domain.clone(),
                    meet_room,
                    meet_user,
                    !is_student,
                );
                Some(html! {
                    <div class="col-sm-12 col-md-12 col-lg-6">
                        { VNode::VRef(self.node.clone()) }
                    </div>
                })
            })
            .unwrap_or_default();

        maybe_meet
    }
}

#[wasm_bindgen(module = "/src/meet.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_meet")]
    fn render_meet_js(node: &Node, domain: String, room: String, user: String, is_moderator: bool);
}

fn render_meet(node: &Node, domain: String, room: String, user: String, is_moderator: bool) {
    render_meet_js(node, domain, room, user, is_moderator)
}