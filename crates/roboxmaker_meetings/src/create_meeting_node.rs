use log::*;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use wasm_bindgen::prelude::{Closure, wasm_bindgen, JsValue};
use yew::{html, Component, ComponentLink, Html, ShouldRender, web_sys::{Node, self}};

pub struct CreateMeetVCalendar {
    link: ComponentLink<Self>,
    props: CreateMeetVCalendarProps,
    node_vcalendar: Node,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct CreateMeetVCalendarProps {
    pub on_callback_date: Callback<String>,
}

#[derive(Debug)]
pub enum CreateMeetVCalendarMessage {
    DateFromVCalendar(String),
}

impl CreateMeetVCalendar {
    fn date_selected_activity(
        &mut self,
        date: String,
        callback: Callback<String>,
    ) {
        let on_res_selected = Closure::wrap(Box::new(move |data: String| {
            callback.emit(data)
        }) as Box<dyn Fn(String)>);
        
        activity_vcalendar(
            &self.node_vcalendar.clone(),
            date,
            &on_res_selected,
        );
        
        on_res_selected.forget();
    }
}

impl Component for CreateMeetVCalendar {
    type Message = CreateMeetVCalendarMessage;
    type Properties = CreateMeetVCalendarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let node_vcalendar = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| {
                let _ = div.set_id("VC-Meet");
                Some(Node::from(div))
            });
            
        CreateMeetVCalendar {
            link,
            props,
            node_vcalendar: node_vcalendar.unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            CreateMeetVCalendarMessage::DateFromVCalendar(date) => {
                self.props.on_callback_date.emit(date.clone());
                info!("DDD {:?}", date.clone()); 
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = true;

        if self.props != props {
            self.props = props;
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { VNode::VRef(self.node_vcalendar.clone()) }
            </div>
        }
    }
    fn rendered(&mut self, first_render: bool) {
        // let day = Local::now().date().day().to_string();
        // let month = Local::now().date().month().to_string();
        // let year = Local::now().date().year().to_string();
        // info!("DDDD {:?}", first_render); 
        let on_res_selected = self.link.callback(move |date| CreateMeetVCalendarMessage::DateFromVCalendar(date));
        // let today = {year} + {"-"} + {&month} + {"-"} + {&day};
        let today = chrono::Local::now().date_naive().to_string();

        if first_render {
            self.date_selected_activity(
                today, 
                on_res_selected
            )
        }
    }
}

#[wasm_bindgen(module = "/src/create_meeting_vcalendar.js")]
extern "C" {
    #[wasm_bindgen(js_name = "activity_vcalendar")]
    fn render_calendar_activity_js(node_vcalendar: &Node, today: String, on_res_selected: &JsValue);
}

fn activity_vcalendar(node_vcalendar: &Node, today: String, on_res_selected: &Closure<dyn Fn(String)>,) {
    render_calendar_activity_js(node_vcalendar, today, on_res_selected.as_ref())
}