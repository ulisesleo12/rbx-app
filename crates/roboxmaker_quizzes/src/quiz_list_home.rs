use log::*;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{quiz_model, school_model};
use roboxmaker_types::types::{AppRoute, GroupId, MyUserProfile, SchoolId};
use roboxmaker_loaders::placeholders::card_post_placeholder::CardPostPlaceholder;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};


#[derive(Debug, Clone)]
enum LoadQuizFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadQuizzes {
    Loading,
    Load(LoadQuizFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuizProfile {
    pub quiz_id: Uuid,
    pub title: String,
    pub created_at: String,
}

pub struct QuizzesListHome {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    list_task: Option<SubscriptionTask>,
    quiz_list: Vec<QuizProfile>,
    list_state: LoadQuizzes,

}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchQuizzesByGroupId,
    QuizResp(Option<quiz_model::quizzes_by_group_id::ResponseData>),
}

impl Component for QuizzesListHome {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchQuizzesByGroupId);

        QuizzesListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            list_task: None,
            quiz_list: vec![],
            list_state: LoadQuizzes::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizzesListHome: {:?}", msg);
        let should_render = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            Message::FetchQuizzesByGroupId => {
                self.list_state = LoadQuizzes::Loading;

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = quiz_model::quizzes_by_group_id::Variables {
                        group_id: group_id.0,
                    };

                    let task = quiz_model::QuizzesByGroupId::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::QuizResp(response)
                    );
                    self.list_task = Some(task);
                }
            }
            Message::QuizResp(response) => {
                self.quiz_list = response
                    .clone()
                    .and_then(|data| Some(data.quizzes_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|data| {

                        let timestamp = data.quiz.created_at;
                        let created_at = get_creation_date(timestamp);

                        QuizProfile {
                            quiz_id: data.quiz.quiz_id,
                            title: data.quiz.title.clone().unwrap_or_default(),
                            created_at,
                        }

                    }).collect();
                if !response.clone().and_then(|data| Some(data.quizzes_group)).unwrap_or(vec![]).is_empty() {
                    self.list_state = LoadQuizzes::Load(LoadQuizFound::Found);
                } else {
                    self.list_state = LoadQuizzes::Load(LoadQuizFound::NotFound);
                }
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
        if self.props.group_id != props.group_id {
            self.link.send_message(Message::FetchQuizzesByGroupId);
        }
                
        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let school_id = self.props.school_id;

        let quizzes_list = self
            .quiz_list
            .iter()
            .map(|item| {
                let quiz_id = item.quiz_id;
                let on_quiz_view = self.link.callback(move |_| Message::AppRoute(AppRoute::Quizzes(school_id, group_id, quiz_id)));

                html! {
                    <div class="card-post-view-home bg-white d-flex flex-column justify-content-between align-items-center p-5 me-5">
                        <a onclick={ &on_quiz_view }>
                            <div class="module-message-post line-clamp-message-post">
                                <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                    { &item.title }
                                </span>
                            </div>
                        </a>
                        <a class="w-100" onclick={ &on_quiz_view }>
                            <div class="d-flex align-items-center justify-content-between">
                                <span class="text-dark noir-light is-size-14 lh-17 text-truncate col-5 mb-0">
                                    <i class="fas fa-list-ol fa-lg me-3"></i>
                                    { "Evaluación" }
                                </span>
                                <div class="ms-2">
                                    <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                                        <i class="far fa-clock me-1"></i>
                                        <div class="d-flex flex-wrap">
                                            <span class="text-brown noir-light is-size-13 lh-22 ">{ &item.created_at }</span>
                                        </div>
                                    </span>
                                </div>
                            </div>
                        </a>
                    </div>
                }
            }).collect::<Html>();
        let quiz_list = match self.list_state {
            LoadQuizzes::Loading => {
                html! {
                    <>
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                    </>
                }
            },
            LoadQuizzes::Load(LoadQuizFound::Found) => {
                html! {
                    {quizzes_list}
                }
            },
            LoadQuizzes::Load(LoadQuizFound::NotFound) => {
                html! {
                    <div class="text-center">
                        // <p class="is-size-5">{lang::dict("No posts here.")}</p>
                        <p class="is-size-5">{"No hay evaluaciones."}</p>
                    </div>
                }
            },
        };
        html! {
            <div class="d-flex flex-row">   
                { quiz_list }
            </div>
        }
    }
}