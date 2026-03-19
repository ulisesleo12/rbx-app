use log::*;
use uuid::Uuid;
use chrono::Local;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


use roboxmaker_main::lang;
use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{quiz_model, school_model};
use roboxmaker_loaders::placeholders::card_post_list::CardPostListPlaceholder;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, AppRoute, LoadResponseFound, LoadResponse, SchoolId, MyUserProfile};

use crate::{quiz_card::QuizCard, quiz_select::{QuizSelect, QuizSelectOption}};


#[derive(Debug, Clone, PartialEq)]
pub struct QuizProfile {
    pub quiz_id: Uuid,
    pub title: String,
    pub archived: bool,
    pub published: bool,
    pub created_at: String,
    pub on_dropdown_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuizFilter {
    Alls,
    Published,
    Unpublished,
    Archived,
}

pub struct QuizList {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    quiz_sub: Option<SubscriptionTask>,
    quiz_delete_task: Option<RequestTask>,
    quiz_add_task: Option<RequestTask>,
    quiz_list: Vec<QuizProfile>,
    filter: QuizFilter,
    show_dropdown_filter: bool,
    list_response_state: LoadResponse,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub class_name: String,
    pub inventory_group: Option<Uuid>,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchQuizzesByGroupId,
    QuizzesResp(Option<quiz_model::quizzes_list_by_group::ResponseData>),
    ChangeFilter(QuizFilter),
    ShowDropdownFilter,
    RemovePost(Uuid),
    RemoveResp(Option<quiz_model::quiz_group_delete::ResponseData>),
    NewQuiz,
    NewQuizResp(Option<quiz_model::new_quiz::ResponseData>),
    AddQuiz(Uuid),
    QuizAddedResp(Option<quiz_model::quiz_to_group_add::ResponseData>),
    UpdateQuizList(Uuid, bool, bool),
    DeleteQuiz(Uuid),
    DeleteQuizResp(Option<quiz_model::delete_quiz_by_id::ResponseData>),
}

impl Component for QuizList {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchQuizzesByGroupId);

        QuizList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            quiz_sub: None,
            quiz_delete_task: None,
            quiz_add_task: None,
            quiz_list: vec![],
            filter: QuizFilter::Alls,
            show_dropdown_filter: false,
            list_response_state: LoadResponse::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizList: {:?}", msg);
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            Message::FetchQuizzesByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = quiz_model::quizzes_list_by_group::Variables {
                        group_id: group_id.0,
                    };

                    let task = quiz_model::QuizzesListByGroup::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::QuizzesResp(response)
                        },
                    );
                    self.quiz_sub = Some(task);
                }
            }
            Message::QuizzesResp(response) => {
                self.quiz_list = response
                    .clone()
                    .and_then(|data| Some(data.quizzes_group))
                    .unwrap_or_default()
                    .iter()
                    .filter(|posts| {

                        let archived = posts.archived.clone();
                        let published = posts.published.clone();

                        self.filter == QuizFilter::Alls && archived == false ||
                
                        self.filter == QuizFilter::Published && published == true && archived == false ||
        
                        self.filter == QuizFilter::Unpublished && archived == false && published == false ||
        
                        self.filter == QuizFilter::Archived && archived == true && published == false
                    })
                    .map(|item| {
                        let quiz = item.clone();
                        
                        let timestamp = quiz.quiz.created_at.and_utc().naive_local();
                        let time_fn = get_creation_date(timestamp);
                        
                        QuizProfile {
                            quiz_id: quiz.quiz_id,
                            title: quiz.quiz.clone().title.clone().unwrap_or_default(),
                            archived: quiz.archived,
                            published: quiz.published,
                            created_at: time_fn,
                            on_dropdown_menu: false,
                        }
                    }).collect();

                if !response.clone().and_then(|data| Some(data.quizzes_group)).unwrap_or(vec![]).is_empty() {
                    self.list_response_state = LoadResponse::Load(LoadResponseFound::Found);
                } else {
                    self.list_response_state = LoadResponse::Load(LoadResponseFound::NotFound);
                }

            }
            Message::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                self.link.send_message(Message::FetchQuizzesByGroupId);
            }
            Message::ShowDropdownFilter => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            Message::NewQuiz => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = quiz_model::new_quiz::Variables {
                        quiz_id: Uuid::new_v4(),
                        group_id: self.props.group_id.0,
                        title: None,
                        description: None,
                        created_at: Local::now().naive_local(),
                        is_evaluation: true,
                    };

                    let task = quiz_model::NewQuiz::request(
                        graphql_task,
                        &self.link,
                        vars, 
                        |response| Message::NewQuizResp(response),
                    );
                    self.quiz_add_task = Some(task);
                    self.link.send_message(Message::FetchQuizzesByGroupId);
                }
            }
            Message::NewQuizResp(response) => {
                if response.is_some() {
                    info!("NewQuizResp: {:?}", response.clone().unwrap());
                    if let Some(quiz_id) = response.clone().unwrap().insert_quizzes_group_one.clone().and_then(|item|Some(item.quiz_id)) {
                        self.link.send_message(Message::AppRoute(AppRoute::Quizzes(self.props.school_id, self.props.group_id, quiz_id)));
                    }
                }
            }
            Message::RemovePost(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = quiz_model::quiz_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        quiz_id: quiz_id,
                    };

                    let task = quiz_model::QuizGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::RemoveResp(response)
                    );
                    self.quiz_delete_task = Some(task);
                }
            }
            Message::RemoveResp(response) => {
                if response.is_some() {
                    info!("RemoveResp: {:?}", response.unwrap());
                }
            }
            Message::DeleteQuiz(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::delete_quiz_by_id::Variables { 
                        quiz_id
                    };

                    let task = quiz_model::DeleteQuizById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::DeleteQuizResp(response)
                        },
                    );
                    self.quiz_delete_task = Some(task);
                }
            }
            Message::DeleteQuizResp(response) => {
                if response.is_some() {
                    info!("DeleteQuizResp: {:?}", response.unwrap());
                }
            }
            Message::AddQuiz(quiz_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = quiz_model::quiz_to_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        quiz_id: quiz_id,
                    };

                    let task = quiz_model::QuizToGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::QuizAddedResp(response)
                    );
                    self.quiz_add_task = Some(task);
                }
            }
            Message::QuizAddedResp(response) => {
                if response.is_some() {
                    info!("QuizAddedResp: {:?}", response.clone().unwrap());
                    if let Some(quiz_id) = response.clone().unwrap().insert_quizzes_group_one.clone().and_then(|item|Some(item.quiz_id)) {
                        self.link.send_message(Message::AppRoute(AppRoute::Quizzes(self.props.school_id, self.props.group_id, quiz_id)));
                    }
                }
            }
            Message::UpdateQuizList(quiz_id, published , archived) => {
                for quiz in self.quiz_list.iter_mut() {
                    if quiz.quiz_id == quiz_id {
                        quiz.archived = archived;
                        quiz.published = published;
                    }
                }
            }
        }
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
        let on_alls = self.link.callback(|_| Message::ChangeFilter(QuizFilter::Alls));
        let on_published = self.link.callback(|_| Message::ChangeFilter(QuizFilter::Published));
        let on_unpublished = self.link.callback(|_| Message::ChangeFilter(QuizFilter::Unpublished));
        let on_archived = self.link.callback(|_| Message::ChangeFilter(QuizFilter::Archived));
        let on_dropdown = self.link.callback(|_| Message::ShowDropdownFilter);
        let on_change_list = self.link.callback(|(quiz_id, published, archived)| Message::UpdateQuizList(quiz_id, published, archived));

        let on_quiz_delete = self.link.callback(|quiz_id| Message::RemovePost(quiz_id));
        let on_quiz_delete_entirely = self.link.callback(|quiz_id| Message::DeleteQuiz(quiz_id));

        let quiz_option_seleted = match self.filter {
            QuizFilter::Alls => "Everyone",
            QuizFilter::Published => "Released",
            QuizFilter::Unpublished => "Unpublished",
            QuizFilter::Archived => "Archived",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };

        let dropdown_by_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(quiz_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == QuizFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == QuizFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_published>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == QuizFilter::Published {true} else {false}} />
                                        <span class={if self.filter == QuizFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_unpublished>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == QuizFilter::Unpublished {true} else {false}} />
                                        <span class={if self.filter == QuizFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_archived>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == QuizFilter::Archived {true} else {false}} />
                                        <span class={if self.filter == QuizFilter::Archived {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Archived")}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let quizzes_list = self.quiz_list.iter()
            .map(|item| {            
            html! {
                <QuizCard 
                    group_id={ self.props.group_id }
                    school_id={ self.props.school_id }
                    on_app_route={ self.props.on_app_route.clone() }
                    user_profile={ self.props.user_profile.clone() }
                    on_quiz_delete={ on_quiz_delete.clone() }
                    on_quiz_delete_entirely={ on_quiz_delete_entirely.clone() }
                    on_change_list={ on_change_list.clone() }
                    quiz_profile={ item.clone() }
                />
            }
        }).collect::<Html>();

        let quiz_option = match self.list_response_state {
            LoadResponse::Loading => {
                html! {
                    <div class="d-flex flex-wrap">
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                        <CardPostListPlaceholder />
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap">
                        {quizzes_list}
                    </div>
                }
            },
            LoadResponse::Load(LoadResponseFound::NotFound) => {
                html! {
                    <div class="text-center">
                        // <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No posts here.")}</span>
                        <span class="text-gray-strong is-size-18 lh-20">{"No hay evaluaciones."}</span>
                    </div>
                }
            },
        };
        let quiz_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    QuizSelectOption::Quiz(quiz_id) => Message::AddQuiz(quiz_id)
                });
                // if item.user_staff.is_some() || item.user_teacher.is_some() {
                if item.user_staff.is_some() {
                    Some(html! {
                        <QuizSelect on_select=on_select 
                            allow_create=true
                            school_id=self.props.school_id
                            group_id=group_id.clone()
                            user_profile=self.props.user_profile.clone()
                            on_app_route=self.props.on_app_route.clone() />
                    })
                } else {
                    // Some(html! {
                    //     <SearchPostsGroup on_app_route=self.props.on_app_route.clone()
                    //         user_profile=self.props.user_profile.clone()
                    //         group_id=self.props.group_id
                    //         school_id=self.props.school_id />
                    // })
                    None
                }
            })
            .unwrap_or(html! {});
        let user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
        let on_direct_meet = self.link.callback(move |_| Message::AppRoute(AppRoute::MeetDirect(group_id)));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                { quiz_search }
                { user_profile_pic }
            </div>
        };

        let new_quiz = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    let on_select = self.link.callback(move |_| Message::NewQuiz);
                    Some(html! {
                        <a class="button btn-create-card d-flex align-items-center justify-content-center" onmousedown=on_select.clone()>
                            <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                                <i class="fas fa-plus me-2"></i>
                                <span>{ "Nueva Evaluación" }</span>
                            </span>
                        </a>
                    })
                } else {Some(html! {})}
            })
            .unwrap_or(html! {});

        let dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {"Evaluaciones"} <span class="ps-1">{"("}{self.quiz_list.iter().cloned().len()}{")"}</span>
                </span>
                <div class="d-flex flex-wrap">
                    { dropdown_by_user }
                    { new_quiz }
                </div>
            </div>
        };
        html! {
            <>
                <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7">
                    { head_section }
                    { dropdown }
                    { quiz_option }
                </div>
            </>
        }
    }
}