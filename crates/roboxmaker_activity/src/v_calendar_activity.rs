use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use crate::activity_list::ActivityProfile;
use wasm_bindgen::prelude::{Closure, wasm_bindgen, JsValue};
use yew::{html, Component, ComponentLink, Html, ShouldRender, web_sys::{Node, self}};

pub struct ActivityCardVCalendar {
    link: ComponentLink<Self>,
    props: ActivityCardVCalendarProps,
    node_vcalendar: Node,
    id: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ActivityCardVCalendarProps {
    pub activity_profile: Option<ActivityProfile>,
    pub on_callback_date: Callback<String>,
}

#[derive(Debug)]
pub enum ActivityCardVCalendarMessage {
    DateFromVCalendar(String),
}

impl ActivityCardVCalendar {
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
            self.id.clone(),
            &on_res_selected,
        );
        
        on_res_selected.forget();
    }
}

impl Component for ActivityCardVCalendar {
    type Message = ActivityCardVCalendarMessage;
    type Properties = ActivityCardVCalendarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut id = Uuid::new_v4().to_string();
        id = "Vcalendar".to_string() + &id;
        let node_vcalendar = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| {
                let _ = div.set_id(&id);
                Some(Node::from(div))
            });
            
        ActivityCardVCalendar {
            link,
            props,
            node_vcalendar: node_vcalendar.unwrap(),
            id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ActivityCardVCalendarMessage::DateFromVCalendar(date) => {
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
        let on_res_selected = self.link.callback(move |date| ActivityCardVCalendarMessage::DateFromVCalendar(date));
        // let today = {year} + {"-"} + {&month} + {"-"} + {&day};
        let today = self.props.activity_profile.clone().and_then(|data| Some(format! ("{}", data.deliver))).unwrap_or("01/01/2022".to_string());

        if first_render {
            self.date_selected_activity(
                today, 
                on_res_selected
            )
        }
    }
}

#[wasm_bindgen(module = "/src/activity-v-calendar.js")]
extern "C" {
    #[wasm_bindgen(js_name = "activity_vcalendar")]
    fn render_calendar_activity_js(node_vcalendar: &Node, today: String, id: String, on_res_selected: &JsValue);
}

fn activity_vcalendar(node_vcalendar: &Node, today: String, id: String, on_res_selected: &Closure<dyn Fn(String)>,) {
    render_calendar_activity_js(node_vcalendar, today, id, on_res_selected.as_ref())
}