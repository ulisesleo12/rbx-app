use log::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::{prelude::*, virtual_dom::VNode, web_sys::{self, Node}};


use crate::{quiz_handle::{generate_question_statistics, score_distribution, QuestionStats, QuizUserAnswer}};


pub struct QuizCharts {
    props: Props,
    questions_node: Node,
    n_users_node: Node,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz_user_answer: Option<QuizUserAnswer>,
}

#[derive(Debug)]
pub enum Message {}

impl Component for QuizCharts {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let questions_node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));

        let n_user_node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| Some(Node::from(div)));

        QuizCharts { 
            props,
            questions_node: questions_node.unwrap(),
            n_users_node: n_user_node.unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizCharts: {:?}", msg);
        // match msg {}
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        trace!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true;
        }
        should_render
    }
    fn view(&self) -> Html {

        html! {
            <>
                <div class="pb-5 pt-5">
                    <div class="card" style="width: 100%;">
                        <div class="card-body">
                            { VNode::VRef(self.questions_node.clone()) }    
                        </div>
                    </div>
                </div>
                <div class="pb-5 pt-5">
                    <div class="card" style="width: 100%;">
                        <div class="card-body">
                            { VNode::VRef(self.n_users_node.clone()) }    
                        </div>
                    </div>
                </div>
            </>
        }
    }
    fn rendered(&mut self, _first_render: bool) {
        if let Some(quiz_user_answer) = &self.props.quiz_user_answer {
            // Clean the node content before rendering
            if let Some(element) = self.questions_node.dyn_ref::<web_sys::Element>() {
                element.set_inner_html(""); // Clean the previous content
            }
            let questions_stats = generate_question_statistics(quiz_user_answer);
            render_questions(&self.questions_node, questions_stats); 
            

            // Clean the node content before rendering
            if let Some(element) = self.n_users_node.dyn_ref::<web_sys::Element>() {
                element.set_inner_html(""); // Clean the previous content
            }
            let scores_stats = score_distribution(quiz_user_answer);
            render_n_users(&self.n_users_node, scores_stats); 
        }
    }
}

#[wasm_bindgen(module = "/src/questions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_questions")]
    fn render_questions_js(node: &Node, questions: JsValue);

    #[wasm_bindgen(js_name = "render_n_users")]
    fn render_n_users_js(node: &Node, scores: JsValue);
}

fn render_questions(node: &Node, questions: Vec<QuestionStats>) {
    let to_value = serde_wasm_bindgen::to_value(&questions).unwrap();
    render_questions_js(node, to_value)
}

fn render_n_users(node: &Node, scores: Vec<(i64, usize)>) {
    let to_value = serde_wasm_bindgen::to_value(&scores).unwrap();
    render_n_users_js(node, to_value)
}