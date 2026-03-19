use log::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys::{Node, self};
use crate::user_messages::MessagesContent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


pub struct MessageReply {
    props: MessageReplyProperties,
    node_reply: Option<Node>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MessageReplyProperties {
    pub content_reply: String,
    pub user_message_profile: Option<MessagesContent>,
}

#[derive(Debug)]
pub enum MessageReplyMessage {
}

impl Component for MessageReply {
    type Message = MessageReplyMessage;
    type Properties = MessageReplyProperties;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let content_reply = props.content_reply.clone();
        let node_reply = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| {
                div.set_class_name("ck-content");
                div.set_inner_html(&content_reply);
                Some(Node::from(div))
            });
        MessageReply {
            props,
            node_reply,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        match msg {
        }
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
        let node_reply = if let Some(node) = &self.node_reply {
            VNode::VRef(node.clone())
        } else {
            html! {
                <span class="icon is-medium">
                    <i class="fas fa-spinner fa-pulse"></i>
                </span>
            }
        };
        html! {
            {node_reply}
        }
    }
}