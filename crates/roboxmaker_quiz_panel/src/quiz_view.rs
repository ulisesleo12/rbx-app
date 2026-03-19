use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};


use crate::{quiz_handle::{quiz_with_user_answers_by_group, QuizUserAnswer}, quiz_metrics::QuizMetrics, quiz_panel::QuizData};


use roboxmaker_models::quiz_model;
use roboxmaker_types::types::{Groups, MyUserProfile, SchoolId, Schools};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask};



#[derive(Debug, Clone, PartialEq, Default)]
pub struct UsersByGroupData {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
}

pub struct QuizView {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    req_quiz_data_task: Option<SubscriptionTask>,
    req_users_task: Option<RequestTask>,
    school_selected: Option<SchoolId>,
    show_dropdown_school: bool,
    show_accordion: bool,
    selected_group_ids: Vec<Uuid>,
    users_by_group_id: Vec<UsersByGroupData>,
    quiz_user_answer: Option<QuizUserAnswer>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub user_profile: Option<MyUserProfile>,
    pub quiz_data: QuizData,
    pub school_id: Option<Uuid>,
    pub go_back: Callback<MouseEvent>,
    pub general_data_schools: Vec<Schools>,
    pub general_data_groups: Vec<Groups>,
}

#[derive(Debug)]
pub enum Message {
    LoadGeneralData,
    SchoolChangeData(SchoolId),
    ShowDropdownSchool,
    ShowAccordion,
    GroupToggleSelect(Uuid),
    FetchUsersByGroupId,    
    RespUsersByGroupId(Option<quiz_model::users_by_group_id::ResponseData>),
    FetchQuizWithUserAnswersByUser,   
    RespQuizWithUserAnswersByUser(Option<quiz_model::quiz_with_user_answers_by_user::ResponseData>),
}

impl Component for QuizView {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::LoadGeneralData);

        QuizView { 
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_users_task: None,
            req_quiz_data_task: None,
            school_selected: None,
            show_dropdown_school: false,
            show_accordion: true,
            selected_group_ids: vec![],
            users_by_group_id: vec![],
            quiz_user_answer: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizView: {:?}", msg);
        let should_update = true;
        match msg {
            Message::LoadGeneralData => {
                self.school_selected = match self.props.general_data_schools.first() {
                    Some(school) => Some(school.school_id),
                    None => None,
                };
            }
            Message::SchoolChangeData(school_id) => {
                self.school_selected = Some(school_id);
                self.show_dropdown_school = false;
                self.show_accordion = false;
                self.quiz_user_answer = None;
                self.users_by_group_id = Vec::new();
                self.selected_group_ids = Vec::new();
            }
            Message::ShowDropdownSchool => {
                self.show_dropdown_school = !self.show_dropdown_school;
            }
            Message::ShowAccordion => {
                self.show_accordion = !self.show_accordion;
            }
            Message::GroupToggleSelect(group_id) => {
                if self.selected_group_ids.contains(&group_id) {
                    self.selected_group_ids.retain(|g| *g != group_id);
                } else {
                    self.selected_group_ids.push(group_id);
                }
                self.link.send_message(Message::FetchUsersByGroupId);
            }

            Message::FetchUsersByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(school_id) = self.school_selected {
                        let vars = quiz_model::users_by_group_id::Variables {
                            school_id: school_id.0,
                            group_ids: self.selected_group_ids.clone(),
                        };
        
                        let task = quiz_model::UsersByGroupId::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                Message::RespUsersByGroupId(response)
                            },
                        );
                        self.req_users_task = Some(task);
                    }
                }
            }
            Message::RespUsersByGroupId(response) => {
                if response.is_some() {
                    info!("[QuizMetrics - RespUsersByGroupId: {:?}", response.clone().unwrap().user_profile.len());

                    self.link.send_message(Message::FetchQuizWithUserAnswersByUser);
                }

                self.users_by_group_id = response
                    .clone()
                    .and_then(|item| Some(item.user_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|item| {
                        UsersByGroupData { 
                            user_id: item.user_id, 
                            full_name: item.full_name.clone(), 
                            email: item.email.clone().unwrap_or_default()
                        }
                    }).collect();
            }
            Message::FetchQuizWithUserAnswersByUser => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::quiz_with_user_answers_by_user::Variables {
                        quiz_id: self.props.quiz_data.quiz_id,
                        user_ids: self.users_by_group_id
                            .iter()
                            .map(|item| item.user_id).collect::<Vec<Uuid>>(),
                    };
    
                    let task = quiz_model::QuizWithUserAnswersByUser::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespQuizWithUserAnswersByUser(response)
                        },
                    );
                    self.req_quiz_data_task = Some(task);
                }
            }
            Message::RespQuizWithUserAnswersByUser(response) => {
                if response.is_some() {
                    info!("[QuizMetrics - RespQuizWithUserAnswersByUser: {:?}", response.clone().unwrap().quiz_responses.len());

                    self.quiz_user_answer = Some(quiz_with_user_answers_by_group(response.clone().unwrap()))
                }
            }
        }
        should_update
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
        // DROPDOWN SCHOOLS
        let all_schools = self.props.general_data_schools.iter().map(|school_group| {
            let school_id = school_group.school_id;
            let school_id_select = format!("{:?}", school_group.school_id);
            let on_show_list_degrees = self.link.callback(move |_| Message::SchoolChangeData(school_id));
            let school_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                true
            } else {
                false
            };
            let class_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                "dropdown-item bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
            } else {
                "dropdown-item text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
            };
            html! {
                <li>
                    <a class={ class_selected } onclick={ on_show_list_degrees }>
                        <input class="bg-checkbox me-1 d-flex align-items-center" type="checkbox" value={ school_id_select } checked={ school_selected } />
                        { &school_group.name }
                    </a>
                </li>
            }
        })
        .collect::<Html>();

        let change_school = self.props.general_data_schools.iter().map(|school_group| {
            let school_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                true
            } else {
                false
            };
            let school = if school_selected {
                html! {
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22-2 text-secondary-purple noir-regular is-size-18 lh-22" style="width: 100% !important;">{ &school_group.name }</span>
                }
            } else {
                html! {}
            };
            html! {
                { school }
            }
        })
        .collect::<Html>();

        let class_dropdown_school = if self.show_dropdown_school {
            "btn btn-secondary btn-second-home dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-home dropdown-toggle d-flex align-items-center justify-content-between"
        };

        let class_dropdown_list_school = if self.show_dropdown_school {
            "dropdown-menu dropdown-menu-home show"
        } else {
            "dropdown-menu dropdown-menu-home"
        };

        let on_dropdown_school = self.link.callback(|_| Message::ShowDropdownSchool);

        let dropdown_schools = if self.props.user_profile.clone().and_then(|item| item.user_staff).is_some() {
            html! {
                <div class="dropdown dropdown-h" style="width: 100% !important;">
                    <button class={ class_dropdown_school } type="button" style="width: 100% !important;" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={ on_dropdown_school }>
                        <img src="/icons/school-3.svg" style="height: 22px;" />
                        { change_school }
                    </button>
                    <ul class={ class_dropdown_list_school } aria-labelledby="dropdownMenuButton2" style="width: 100% !important; border: 2px solid #A4A5E3 !important; border-radius: 10px !important;">
                        { all_schools }
                    </ul>
                </div>
            }
        } else {
            html! {}
        };
        // END DROPDOWN SCHOOLS

        
        // DROPDOWN DEGREES
        let alls_class_groups = self
            .props
            .general_data_groups
            .iter()
            .filter(|item| item.school_id == self.school_selected.unwrap_or_default().0)
            .map(|class_group| {
                let group_id = class_group.group_id;
                let class_id_select = format!("{:?}", group_id);
                let on_show_list_degrees = self.link.callback(move |_| Message::GroupToggleSelect(group_id.0));
                
                let class_group_selected = self.selected_group_ids.contains(&class_group.group_id.0);

                let class_selected = if class_group_selected {
                    "bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center me-4"
                } else {
                    "text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center me-4"
                };
                html! {
                    <a class={ class_selected } onclick={ on_show_list_degrees }>
                        <input class="bg-checkbox me-1 d-flex align-items-center" type="checkbox" value={ class_id_select } checked={ class_group_selected } />
                        { &class_group.class_name }
                    </a>
                }
            })
            .collect::<Html>();
        
        let class_accordion = if self.show_accordion {
            "btn-accordion noir-regular is-size-18 text-secondary-purple accordion-button"
        } else {
            "btn-accordion noir-regular is-size-18 text-secondary-purple accordion-button collapsed"
        };

        let class_accordion_collapse = if self.show_accordion {
            "accordion-collapse collapse show"
        } else {
            "accordion-collapse collapse"
        };

        let on_collapse = self.link.callback(|_| Message::ShowAccordion);
        let accordion_view = html! {
            <div class="accordion mt-4" id="accordionExample">
                <div class="accordion-item">
                    <h2 class="accordion-header" id="headingOne">
                        <button class={ class_accordion } type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" 
                            aria-expanded={ if self.show_accordion { "true" } else { "false" } } 
                            aria-controls="collapseOne" onclick={ on_collapse }>
                            { "Seleccionar grado" }
                        </button>
                    </h2>
                    <div id="collapseOne" class={ class_accordion_collapse } aria-labelledby="headingOne" data-bs-parent="#accordionExample">
                        <div class="accordion-body d-flex flex-wrap">
                            { alls_class_groups }
                        </div>
                    </div>
                </div>
            </div>
        };


        let go_back_evaluations = html! {
            <a onclick={ self.props.go_back.clone() } class="mb-2">
                <span class="text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center">
                    <i class="fas fa-arrow-left"></i>
                    <span class="mx-2">{ "A evaluaciones" }</span>
                </span>
            </a>
        };


        let quiz_title = html! {
            <div class="d-flex justify-content-between mt-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 pb-4 mb-1">
                    { &self.props.quiz_data.title }
                </h1>
            </div>
        };

        html! {
            <>
                { go_back_evaluations }
                { quiz_title }
                { dropdown_schools }
                { accordion_view }

                <QuizMetrics quiz_user_answer={ self.quiz_user_answer.clone() } />
            </>
        }
    }
}