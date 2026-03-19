use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{handle, quiz::{Quiz, QuizApp, QuizMode}, quizresponder};


use roboxmaker_models::quiz_model;
use roboxmaker_utils::user_pic::UserPic;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{AppRoute, ClassGroupCategory, GroupId, MyUserProfile, SchoolId, UserId};


pub struct QuizPage {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    task: Option<SubscriptionTask>,
    quiz: Option<Quiz>,
    quiz_state: String,
    class_name: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub quiz_id: Uuid,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub saved_sidebar_state: bool,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchQuizById(Uuid, GroupId),
    Post(Option<quiz_model::quiz_by_id::ResponseData>),
}

impl Component for QuizPage {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchQuizById(props.quiz_id, props.group_id));

        QuizPage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            quiz: None,
            quiz_state: String::new(),
            class_name: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            Message::FetchQuizById(quiz_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::quiz_by_id::Variables { 
                        group_id: group_id.0,
                        quiz_id: quiz_id,
                    };

                    let task = quiz_model::QuizById::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::Post(response)
                    );
                    self.task = Some(task);
                }
            }
            Message::Post(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                if let Some(resp) = response.clone().and_then(|data| data.quizzes_group_by_pk) {
                    self.class_name = resp.class_profile.clone().and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or_default();

                    self.quiz = Some(handle::get_quiz(resp.clone(), user_id.0));
                    if let Some(quiz) = &self.quiz {
                        self.quiz_state = quiz.state.clone();
                    }
                }
                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                
                if response.clone().and_then(|data| data.quizzes_group_by_pk).is_none() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Quizzes)));
                    } else {
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Quizzes)));
                    }
                }
            }
        };
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

        let group_id = self.props.group_id;
        let school_id = self.props.school_id;
        let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

        let on_class_group_posts = self.link.callback(move |_| {
            Message::AppRoute(AppRoute::SchoolGroupSection(
                school_id.clone(),
                group_id.clone(),
                ClassGroupCategory::Quizzes,
            ))
        });
        let on_class_group_posts_st = self.link.callback(move |_| {
            Message::AppRoute(AppRoute::GroupSectionStudent(
                school_id.clone(),
                user_id.clone(),
                ClassGroupCategory::Quizzes,
            ))
        });

        let go_back_grade = self.props.user_profile.clone()
        .and_then(|item| {
            if item.user_teacher.is_some() || item.user_staff.is_some() {
                Some(html! {
                    <a onclick={ on_class_group_posts }>
                        <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
                            <span class="icon">
                                <i class="fas fa-arrow-left"></i>
                            </span>
                            <span class="mx-2">{ "A Evaluaciones" }</span>
                            { self.class_name.clone() }
                        </span>
                    </a>
                })
            } else {
                Some(html! {
                    <a onclick={ on_class_group_posts_st }>
                        <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
                            <span class="icon">
                                <i class="fas fa-arrow-left"></i>
                            </span>
                            <span class="mx-2">{ "A Evaluaciones" }</span>
                            { self.class_name.clone() }
                        </span>
                    </a>
                })
            }
        }).unwrap_or(html! {});

        if let Some(quiz) = self.quiz.clone() {
            let quiz_clone = quiz.clone();
            if self.quiz_state == String::from("CREATED") {
                html! {
                    <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
                        <div class="d-flex align-items-center justify-content-between">
                            { go_back_grade }
                            <div class="d-flex align-items-center flex-wrap">
                                <UserPic  user_profile={ self.props.user_profile.clone() }/>
                            </div>
                        </div>
                        {
                            if let Some(author) = quiz_clone.author_profile {
                                html! {
                                    <div class="mt-7">
                                        <div class="d-flex flex-wrap align-items-center justify-content-between pt-5 mb-2">
                                            <div class="d-flex align-items-center">
                                                <img class="img-card-32" src={ author.pic_path.clone() } alt="" style="height: 32px; object-fit: cover;" />
                                                <span class="text-dark noir-light is-size-18 lh-22 ps-2">{ &author.full_name }</span>
                                            </div>
                                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                                <i class="far fa-clock"></i>
                                                <span class="ps-2">{ &quiz_clone.create_at }</span>
                                            </span>
                                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                                <i class="fas fa-graduation-cap"></i>
                                                <span class="ps-2">{ self.class_name.clone() }</span>
                                            </span>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <quizresponder::QuizResponder 
                            quiz={ quiz.clone() } 
                            user_profile={ self.props.user_profile.clone() }
                            group_id={ self.props.group_id }
                            school_id={ self.props.school_id }
                            quiz_id={ self.props.quiz_id } 
                            />
                    </div>
                }
            } else {
                html! {
                    <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
                        <div class="d-flex align-items-center justify-content-between">
                            { go_back_grade }
                            <div class="d-flex align-items-center flex-wrap">
                                <UserPic  user_profile={ self.props.user_profile.clone() }/>
                            </div>
                        </div>
                        {
                            if let Some(author) = quiz_clone.author_profile {
                                html! {
                                    <div class="mt-7">
                                        <div class="d-flex flex-wrap align-items-center justify-content-between pt-5 mb-2">
                                            <div class="d-flex align-items-center">
                                                <img class="img-card-32" src={ author.pic_path.clone() } alt="" style="height: 32px; object-fit: cover;" />
                                                <span class="text-dark noir-light is-size-18 lh-22 ps-2">{ &author.full_name }</span>
                                            </div>
                                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                                <i class="far fa-clock"></i>
                                                <span class="ps-2">{ &quiz_clone.create_at }</span>
                                            </span>
                                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                                <i class="fas fa-graduation-cap"></i>
                                                <span class="ps-2">{ self.class_name.clone() }</span>
                                            </span>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <QuizApp
                            quiz_mode={ QuizMode::Create }
                            group_id={ self.props.group_id }
                            school_id={ self.props.school_id }
                            quiz_id={ self.props.quiz_id }
                            is_evaluation={ self.quiz.clone().and_then(|q| Some(q.is_evaluation)).unwrap_or(false) } />
                    </div>
                }
            }
        } else {
            html! {
                <progress class="progress is-small is-primary" max="100"></progress>
            }
        }

    }
}