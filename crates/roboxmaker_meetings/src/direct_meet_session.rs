use log::*;
use yew::web_sys::Node;
use yew::virtual_dom::VNode;
use wasm_bindgen::prelude::*;
use yew::{prelude::*, web_sys};
use yew::services::fetch::FetchTask;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_types::types::{MyUserProfile, GroupId};

pub struct DirectMeetSession {
    _link: ComponentLink<Self>,
    props: DirectMeetSessionProperties,
    _task: Option<FetchTask>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));
        DirectMeetSession {
            _link: link,
            props,
            _task: None,
            node: node.unwrap(),
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
        let is_student = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| item.user_student.as_ref())
            .is_some();
        let maybe_meet = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let meet_room = self.props.group_id.0.to_string();
                let meet_user = item.full_name.clone();
                render_meet(
                    &self.node,
                    self.props.domain.clone(),
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