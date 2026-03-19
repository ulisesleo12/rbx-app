use log::*;
use yew::prelude::*;
use web_sys::{Node, self};
use yew::virtual_dom::VNode;
use yew::{html, Component, Html};
use crate::user_messages::MessagesContent;


pub struct MessageReply {
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

    fn create(ctx: &Context<Self>) -> Self {
        let content_reply = ctx.props().content_reply.clone();
        let node_reply = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| {
                div.set_class_name("ck-content");
                div.set_inner_html(&content_reply);
                Some(Node::from(div))
            });
        MessageReply {
            node_reply,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        match msg {
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props() != old_props {
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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