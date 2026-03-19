use log::*;
use uuid::Uuid;
use yew::prelude::*;
use chrono::NaiveDateTime;
use code_location::code_location;
use serde_derive::{Deserialize, Serialize};
use roboxmaker_loaders::fullscreen_loader::FullScreenLoader;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


use crate::quiz_view::QuizView;


use roboxmaker_main::lang;
use roboxmaker_models::quiz_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{AppRoute, GroupId, Groups, LoadFullScreen, LoadFullScreenFound, MyUserProfile, SchoolId, Schools};



#[derive(Debug, Clone, PartialEq)]
pub struct QuizData {
    pub quiz_id: Uuid,
    pub title: String,
    pub state: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupData {
    pub class_name: String,
    pub group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSchool {
    pub name: String,
    pub inventory_group: Uuid,
    pub school_id: SchoolId,
}

pub struct QuizzesPanel {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    req_quizzes_task: Option<RequestTask>,
    loading_screen: LoadFullScreen,
    quizzes: Vec<QuizData>,
    quiz_selected: Option<QuizData>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub general_data_schools: Vec<Schools>,
    pub general_data_groups: Vec<Groups>,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchQuizzes,
    Quizzes(Option<quiz_model::get_quizzes::ResponseData>),
    QuizSelect(Option<QuizData>),
}

impl Component for QuizzesPanel {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchQuizzes);

        QuizzesPanel { 
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_quizzes_task: None,
            loading_screen: LoadFullScreen::Loading,
            quizzes: vec![],
            quiz_selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizzesPanel: {:?}", msg);
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            Message::FetchQuizzes => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::get_quizzes::Variables {};

                    let task = quiz_model::GetQuizzes::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::Quizzes(response)
                        },
                    );
                    self.req_quizzes_task = Some(task);
                }
            }
            Message::Quizzes(response) => {
                if !response.clone().and_then(|data| Some(data.quizzes)).unwrap_or(vec![]).is_empty() {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::Found);
                } else {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::NotFound);
                }

                self.quizzes = response
                    .clone()
                    .and_then(|data| Some(data.quizzes))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|item| {

                        QuizData { 
                            quiz_id: item.quiz_id, 
                            title: item.title.clone().unwrap_or(String::from("Sin título")), 
                            state: item.state.clone().unwrap_or_default(),
                            created_at: item.created_at,
                        }
                    })
                    .collect();
            }
            Message::QuizSelect(quiz) => {
                self.quiz_selected = quiz
            }
        }
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
        // info!("QuizzesPanelGeneralData: {:?}", self.props.general_data);
        // info!("QuizzesPanelGeneralData: {:?}", self.props.general_data_schools);
        // info!("QuizzesPanelGeneralData: {:?}", self.props.general_data_groups);


        let welcome_class_view = self.props.user_profile.as_ref().and_then(|user_profile| {
            Some(html! {
                <div class="d-flex justify-content-between">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 pb-4 mb-1">{lang::dict("Hello, ")}
                        { &user_profile.full_name }
                    </h1>
                </div>
            })
        }).unwrap_or(html! {});


        let quiz_list = self.quizzes.iter().map(|item| {
            let quiz_clone = item.clone();
            html! {
                <div class="pb-5 cursor-pointer" onclick={ self.link.callback(move |_| Message::QuizSelect(Some(quiz_clone.clone()))) }>
                    <div class="meetings-container-card-false d-flex align-items-center justify-content-between px-5">
                        <div class="d-flex flex-wrap">
                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{ &item.title }</span>
                        </div>
                    </div>
                </div>
            }
        }).collect::<Html>();

        let quiz_view = if self.quiz_selected.is_some() {
            let go_back = self.link.callback(move |_| Message::QuizSelect(None));
            html! {
                <QuizView 
                    user_profile={ self.props.user_profile.clone() }
                    quiz_data={ self.quiz_selected.clone().unwrap() }
                    go_back={ go_back }
                    general_data_schools={ self.props.general_data_schools.clone() }
                    general_data_groups={ self.props.general_data_groups.clone() }  />
            }
        } else {
            html! {
                <>
                    { welcome_class_view }
                    { quiz_list }
                </>
            }
        };

        match self.loading_screen {
            LoadFullScreen::Loading => {
                html! {
                    <FullScreenLoader />
                }
            },
            LoadFullScreen::Load(LoadFullScreenFound::Found) => {
                html! {
                    <>
                        <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                            <div class="w-100 pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">

                                { quiz_view }
                            
                            </div>
                        </div>
                    </>
                }
            },
            LoadFullScreen::Load(LoadFullScreenFound::NotFound) => {
                html! {
                    <FullScreenLoader />
                }
            },
        }
    }
}