use log::*;
use uuid::Uuid;
use web_sys::Node;
use wasm_bindgen::prelude::*;
use yew::{virtual_dom::VNode, Context};
use yew::{Callback, Component, Html, Properties};

use roboxmaker_types::types::{MyUserProfile, UserId};

pub struct CKEditor {
    node: Node,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct CKEditorProps {
    pub user_profile: Option<MyUserProfile>,
    pub content: String,
    pub upload_url: String,
    pub on_data: Option<Callback<String>>,
}

#[derive(Debug)]
pub enum Message {}

impl Component for CKEditor {
    type Message = Message;
    type Properties = CKEditorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        // Creating an element that we can render the React component into later
        let node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));

        CKEditor {
            node: node.unwrap(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        trace!("{:?} => {:?} ", ctx.props(), old_props);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user_id = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user| Some(user.user_id))
            .unwrap_or(UserId(Uuid::default()));
        render_ckeditor(
            &self.node,
            ctx.props().content.clone(),
            user_id.to_string(),
            ctx.props().upload_url.clone(),
            ctx.props().on_data.clone(),
        );
        VNode::VRef(self.node.clone())
    }
}

#[wasm_bindgen(module = "/src/ckeditor.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_ckeditor")]
    fn render_ckeditor_js(node: &Node, content: String, user_id: String, upload_url: String, on_data: JsValue);
}

fn render_ckeditor(
    node: &Node,
    content: String,
    user_id: String,
    upload_url: String,
    on_data: Option<Callback<String>>,
) {
    let callback = Closure::once_into_js(move |data| {
        if let Some(on_data) = on_data {
            on_data.emit(data)
        }
    });
    render_ckeditor_js(node, content, user_id, upload_url, callback)
}
