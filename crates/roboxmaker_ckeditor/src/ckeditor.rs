use log::*;
use uuid::Uuid;
use yew::virtual_dom::VNode;
use wasm_bindgen::prelude::*;
use yew::web_sys::{Node, self};
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender};

use roboxmaker_types::types::{MyUserProfile, UserId};

pub struct CKEditor {
    _link: ComponentLink<Self>,
    node: Node,
    props: CKEditorProps,
}

#[derive(Debug, Properties, Clone)]
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Creating an element that we can render the React component into later
        let node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));
        CKEditor {
            _link: link,
            node: node.unwrap(),
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        trace!("{:?} => {:?}", self.props, props);
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let user_id = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user| Some(user.user_id))
            .unwrap_or(UserId(Uuid::default()));
        render_ckeditor(
            &self.node,
            self.props.content.clone(),
            user_id.to_string(),
            self.props.upload_url.clone(),
            self.props.on_data.clone(),
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
