use log::info;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use chrono::{Local, NaiveDateTime};
use crate::quiz::{AnswerOption, Question, QuestionSection, QuestionType, Quiz, QuizApp, QuizMode, QuizResponses, UserAnswer};


use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, MyUserProfile, QuizResponderPage, SchoolId};
use roboxmaker_models::quiz_model::{self, upsert_quiz_response, UpsertQuizResponse};


pub struct QuizResponder {
    props: QuizResponderProps,
    link: ComponentLink<Self>,
    graphql_task: Option<GraphQLTask>,
    start_quiz_task: Vec<RequestTask>,
    single_answer_task: Vec<RequestTask>,
    multi_answer_task: Vec<RequestTask>,
    boolean_answer_task: Vec<RequestTask>,
    text_answer_task: Vec<RequestTask>,
    answers: Vec<(Uuid, NaiveDateTime, UserAnswer)>,
    quiz_resp_page: QuizResponderPage,
    validate_all_questions_answered: bool,
    quiz_response: Option<QuizResponses>,
    quiz_response_id: Uuid,
    quiz_response_status: String,
    sending_response: bool,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct QuizResponderProps {
    pub quiz: Quiz,
    pub user_profile: Option<MyUserProfile>,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub quiz_id: Option<Uuid>,

}

#[derive(Debug)]
pub enum ResponderMsg {
    FIllVars,
    UpsertQuiz,
    UpsertQuizResponse(Option<quiz_model::upsert_quiz_response::ResponseData>),
    ChangePageMode(QuizResponderPage),
    SelectSingleChoice(Question, Uuid, Uuid), 
    ToggleMultipleChoice(Question, Uuid, Uuid), 
    SetTrueFalse(Question, Uuid, Uuid, bool),
    SetTextAnswer(Question, Uuid, String),
    SubmitQuiz,
    SubmitQuizSingleResponse(Option<quiz_model::submit_single_answer::ResponseData>),
    SubmitQuizMultiResponse(Option<quiz_model::submit_multi_answer::ResponseData>),
}

impl Component for QuizResponder {
    type Message = ResponderMsg;
    type Properties = QuizResponderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // info!("{:?}", props.quiz);
        link.send_message(ResponderMsg::FIllVars);

        QuizResponder {
            props,
            link,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            start_quiz_task: vec![],
            single_answer_task: vec![],
            multi_answer_task: vec![],
            boolean_answer_task: vec![],
            text_answer_task: vec![],
            answers: Vec::new(),
            quiz_resp_page: QuizResponderPage::None,
            validate_all_questions_answered: false,
            quiz_response: None,
            quiz_response_id: Uuid::default(),
            quiz_response_status: String::new(),
            sending_response: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizResponder: {:?}", msg);
        match msg {
            ResponderMsg::FIllVars => {
                self.quiz_response = self.props.quiz.quiz_responses.first().cloned();
        
                self.quiz_response_id = if self.quiz_response.is_some() {
                    self.quiz_response.clone().unwrap().quiz_response_id
                } else {
                    Uuid::new_v4()
                };
                self.quiz_response_status = if self.quiz_response.is_some() {
                    String::from("FILLED")
                } else {
                    String::from("IN_PROGRESS")
                };
                true
            }
            ResponderMsg::UpsertQuiz => {
                if let Some(user_profile) = &self.props.user_profile {

                    let quiz_response = self.quiz_response.clone().unwrap_or(QuizResponses::default());

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = upsert_quiz_response::Variables { 
                            quiz_response_id: self.quiz_response_id,
                            quiz_id: self.props.quiz.id,
                            user_id: user_profile.user_id.0,
                            status: self.quiz_response_status.clone(),
                            completed_at: Local::now().naive_local(),
                            started_at: if self.quiz_response_status == String::from("IN_PROGRESS") { Local::now().naive_local() } else { quiz_response.started_at }
                        };
    
                        let task = UpsertQuizResponse::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ResponderMsg::UpsertQuizResponse(response)
                            },
                        );
                        self.start_quiz_task.push(task);
                    }
                }
                false
            }
            ResponderMsg::UpsertQuizResponse(response) => {
                self.sending_response = false;
                if response.as_ref().and_then(|resp| resp.insert_quiz_responses_one.as_ref() ).is_some() {
                    self.quiz_response_status = String::from("IN_PROGRESS");
                    self.validate_all_questions_answered = false;
                    self.start_quiz_task = vec![];
                    self.single_answer_task = vec![];
                    self.multi_answer_task = vec![];
                    self.boolean_answer_task = vec![];
                    self.text_answer_task = vec![];
                    self.answers = Vec::new();
                    info!("UpsertQuizResponse: {:?}", response.unwrap());
                } else {
                    log::error!("Failed to Start Quiz")
                }
                true
            }
            ResponderMsg::ChangePageMode(quiz_resp_page) => {
                self.quiz_resp_page = quiz_resp_page;
                true
            }
            ResponderMsg::SelectSingleChoice(question, question_id, option_id) => {
                let answer = UserAnswer::SingleChoice(question, option_id);
                self.update_answer(question_id, answer);

                let user_answers = self.answers.iter().map(|item| item.2.clone()).collect();
                self.validate_all_questions_answered = crate::quizfunctions::validate_all_questions_answered(&self.props.quiz, &user_answers);
                true
            }
            ResponderMsg::ToggleMultipleChoice(question, question_id, option_id) => {
                if let Some((_, _, UserAnswer::MultipleChoice(_, current_answers))) = 
                    self.answers.iter_mut().find(|(id, _, _)| *id == question_id) 
                {
                    if let Some(pos) = current_answers.iter().position(|id| id == &option_id) {
                        current_answers.remove(pos);
                    } else {
                        current_answers.push(option_id);
                    }
                } else {
                    let answer = UserAnswer::MultipleChoice(question, vec![option_id]);
                    self.update_answer(question_id, answer);
                }

                let user_answers = self.answers.iter().map(|item| item.2.clone()).collect();
                self.validate_all_questions_answered = crate::quizfunctions::validate_all_questions_answered(&self.props.quiz, &user_answers);
                true
            }
            ResponderMsg::SetTrueFalse(question, question_id, option_id, answer) => {
                let answer = UserAnswer::TrueFalse(question, option_id, answer);
                self.update_answer(question_id, answer);

                let user_answers = self.answers.iter().map(|item| item.2.clone()).collect();
                self.validate_all_questions_answered = crate::quizfunctions::validate_all_questions_answered(&self.props.quiz, &user_answers);
                true
            }
            ResponderMsg::SetTextAnswer(question, question_id, text) => {
                let answer = UserAnswer::TextInput(question, text.clone());
                self.update_answer(question_id, answer);

                let user_answers = self.answers.iter().map(|item| item.2.clone()).collect();
                self.validate_all_questions_answered = crate::quizfunctions::validate_all_questions_answered(&self.props.quiz, &user_answers);

                self.validate_all_questions_answered = !text.clone().is_empty();
                
                true
            }
            ResponderMsg::SubmitQuiz => {
                if self.validate_all_questions_answered && !self.sending_response {
                    if let Some(graphql_task) = self.graphql_task.as_mut() {

                        if let Some(quiz_response) = self.props.quiz.quiz_responses.first() {
                            self.sending_response = true;
                            let quiz_response_id = quiz_response.quiz_response_id;

                            for item  in self.answers.iter() {
                                match item.2.clone() {
                                    UserAnswer::SingleChoice(_question, option_id) => { 
                                        let vars = quiz_model::submit_single_answer::Variables {
                                            answer_type: String::from("SINGLE_CHOICE"),
                                            answered_at: item.1,
                                            is_true: None,
                                            question_id: item.0,
                                            quiz_response_id,
                                            score: Some(0),
                                            single_choice_option_id: Some(option_id),
                                            text_answer: None,
                                            user_answer_id: Uuid::new_v4(),
                                        };
                                        
                                        let task = quiz_model::SubmitSingleAnswer::request(
                                            graphql_task,
                                            &self.link,
                                            vars,
                                            |response| {
                                                ResponderMsg::SubmitQuizSingleResponse(response)
                                            },
                                        );
                                        self.single_answer_task.push(task);
                                    },
                                    UserAnswer::MultipleChoice(_question, options) => { 
                                        let user_answer_id = Uuid::new_v4();
                                        let multiple_choice = options.iter().map(|option_id| {
                                            quiz_model::submit_multi_answer::UserMultipleChoicesInsertInput {
                                                answer_option: None, 
                                                id: Some(Uuid::new_v4()), 
                                                option_id: Some(option_id.clone()), 
                                                user_answer_id: Some(user_answer_id),
                                            }
                                        }).collect();
    
                                        let vars = quiz_model::submit_multi_answer::Variables {
                                            answer_type: String::from("MULTIPLE_CHOICE"),
                                            answered_at: item.1,
                                            is_true: None,
                                            question_id: item.0,
                                            quiz_response_id,
                                            score: Some(0),
                                            single_choice_option_id: None,
                                            text_answer: None,
                                            user_answer_id,
                                            user_multiple_choices: multiple_choice,
                                        };
                                        
                                        let task = quiz_model::SubmitMultiAnswer::request(
                                            graphql_task,
                                            &self.link,
                                            vars,
                                            |response| {
                                                ResponderMsg::SubmitQuizMultiResponse(response)
                                            },
                                        );
                                        self.multi_answer_task.push(task);
                                    },
                                    UserAnswer::TrueFalse(_question, option_id, _is_true) => { 
                                        let vars = quiz_model::submit_single_answer::Variables {
                                            answer_type: String::from("TRUE_FALSE"),
                                            answered_at: item.1,
                                            is_true: None,
                                            question_id: item.0,
                                            quiz_response_id,
                                            score: Some(0),
                                            single_choice_option_id: Some(option_id),
                                            text_answer: None,
                                            user_answer_id: Uuid::new_v4(),
                                        };
                                        
                                        let task = quiz_model::SubmitSingleAnswer::request(
                                            graphql_task,
                                            &self.link,
                                            vars,
                                            |response| {
                                                ResponderMsg::SubmitQuizSingleResponse(response)
                                            },
                                        );
                                        self.boolean_answer_task.push(task);
                                    },
                                    UserAnswer::TextInput(_question, text_answer) => { 
                                        let vars = quiz_model::submit_single_answer::Variables {
                                            answer_type: String::from("TEXT_INPUT"),
                                            answered_at: item.1,
                                            is_true: None,
                                            question_id: item.0,
                                            quiz_response_id,
                                            score: Some(0),
                                            single_choice_option_id: None,
                                            text_answer: Some(text_answer),
                                            user_answer_id: Uuid::new_v4(),
                                        };
                                        
                                        let task = quiz_model::SubmitSingleAnswer::request(
                                            graphql_task,
                                            &self.link,
                                            vars,
                                            |response| {
                                                ResponderMsg::SubmitQuizSingleResponse(response)
                                            },
                                        );
                                        self.text_answer_task.push(task);
                                    },
                                }
                            };
                            self.link.send_message(ResponderMsg::UpsertQuiz);
                        }
                    }
                    true
                } else {
                    info!("Answers to answer");
                    false
                }
            }
            ResponderMsg::SubmitQuizSingleResponse(response) => {
                if response.is_some() {
                    info!("SubmitQuizSingleResponse: {:?}", response.unwrap());
                } else {
                    log::error!("Failed to Submit Single Response")
                }
                true
            }
            ResponderMsg::SubmitQuizMultiResponse(response) => {
                if response.is_some() {
                    info!("SubmitQuizMultiResponse: {:?}", response.unwrap());
                } else {
                    log::error!("Failed to Submit Multi Response")
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("QuizResponderProps: {:?}", props);

        if self.props.quiz.quiz_responses != props.quiz.quiz_responses {
            self.link.send_message(ResponderMsg::FIllVars)
        }
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let quiz = &self.props.quiz;

        let edit_quiz_btn = self
            .props
            .user_profile
            .clone()
            .and_then(|item| {
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4" style="min-width: 230px;"
                            onclick={ self.link.callback(|_| ResponderMsg::ChangePageMode(QuizResponderPage::Edit)) }>
                            <span class="text-white noir-bold is-size-16 lh-20">{ "Editar" }</span>
                        </button>
                    })
                } else if item.user_student.is_some() {
                    if quiz.quiz_responses.iter().find(|q_resp| q_resp.user_id == item.user_id.0).is_some() {
                        Some(html! {
                            <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4" style="min-width: 230px;"
                                disabled={ !self.validate_all_questions_answered || self.sending_response }
                                onclick={ self.link.callback(|_| ResponderMsg::SubmitQuiz) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Enviar" }</span>
                            </button>
                        })
                    } else {
                        Some(html! {
                            <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4" style="min-width: 230px;"
                                onclick={ self.link.callback(|_| ResponderMsg::UpsertQuiz) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Iniciar" }</span>
                            </button>
                        })
                    }
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let filled_quiz = if self.quiz_response.is_some() {
            if self.quiz_response.clone().unwrap().status == String::from("IN_PROGRESS") {
                String::from("IN_PROGRESS")
            } else {
                String::from("FILLED")
            }
        } else {
            String::from("NONE")
        };

        let user_role = self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some())).unwrap_or(false);

        let view_sections = if filled_quiz != String::from("FILLED") {
            html! {
                <>
                    {
                        if user_role || self.quiz_response.is_some() {
                            html! {
                                <div class="quiz-sections">
                                    { for quiz.sections.iter().map(|section| self.render_section(section)) }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    <div class="d-flex justify-content-center mt-6">
                        { edit_quiz_btn }
                    </div>
                </>
            }
        } else {
            html! {
                <div class="alert bg-primary-blue-light noir-medium text-white mt-4" role="alert">
                    <p class="mb-0">{"La evaluación ha sido completada"}</p>
                </div>
            }
        };

        html! {
            <>
                {
                    match self.quiz_resp_page {
                        QuizResponderPage::None => html! {
                            <div class="quiz-responder mx-4 mt-4 pb-6">
                                <div class="quiz-header mb-4">
                                    <h3 class="text-primary-blue-dark text-camelcase noir-bold is-size-48 text-center lh-43 mb-3">{&quiz.title}</h3>
                                    {
                                        if !quiz.description.is_empty() {
                                            html! { 
                                                <p class="text-gray-dark noir-light is-size-16 text-justify mb-3">{&quiz.description}</p> 
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                                { view_sections }
                            </div>
                        },
                        QuizResponderPage::Edit => html! {
                            <>
                                <QuizApp
                                    quiz={ Some(quiz.clone()) }
                                    quiz_mode={ QuizMode::Edit }
                                    group_id={ self.props.group_id }
                                    school_id={ self.props.school_id }
                                    quiz_id={ self.props.quiz_id }
                                    on_quiz_responder_page={ Some(self.link.callback(move |_| ResponderMsg::ChangePageMode(QuizResponderPage::None))) }
                                    is_evaluation={ self.props.quiz.is_evaluation }  />
                            </>
                        },
                    }
                }
            </>
        }
    }
}

impl QuizResponder {
    fn update_answer(&mut self, question_id: Uuid, answer: UserAnswer) {
        let timestamp = Local::now().naive_local();
        if let Some((_, _, existing_answer)) = self.answers.iter_mut().find(|(id, _, _)| *id == question_id) {
            *existing_answer = answer;
        } else {
            self.answers.push((question_id, timestamp, answer));
        }
    }

    fn render_section(&self, section: &QuestionSection) -> Html {
        html! {
            <div class="section mb-5" key={ section.id.clone().to_string() }>
                <div class="section-header mb-3">
                    <h6 class="text-purple-on noir-medium is-size-18 lh-22">
                        <li>{&section.title}</li>
                    </h6>
                    {
                        if !section.description.is_empty() {
                            html! { 
                                <p class="text-gray-strong noir-light is-size-16 text-justify mb-3">{&section.description}</p> 
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div class="questions">
                    { for section.questions.iter().map(|question| self.render_question(question)) }
                </div>
            </div>
        }
    }

    fn render_question(&self, question: &Question) -> Html {
        let current_answer = self.answers.iter()
            .find(|(id, _, _)| *id == question.id)
            .map(|(_, _, answer)| answer);

        let user_role = self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some())).unwrap_or(false);

        html! {
            <div class="question mb-4" key={ question.id.clone().to_string() }>
                <div class="question-header">
                    <label class="text-primary-blue-light noir-light is-size-16 mb-3">{ &question.text }</label>
                    {
                        if user_role {
                            html! {
                                <div>
                                    <span class="badge bg-gray-strong text-white noir-medium">
                                        {
                                            match question.question_type {
                                                QuestionType::SingleChoice => "Selección única",
                                                QuestionType::MultipleChoice => "Selección múltiple",
                                                QuestionType::TrueFalse => "Verdadero/Falso",
                                                QuestionType::TextInput => "Respuesta textual",
                                            }
                                        }
                                    </span>
                                    <span class="badge bg-light-sea-green text-white noir-medium ms-2">
                                        { format!("{} puntos", question.points) }
                                    </span>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                {
                    match question.question_type {
                        QuestionType::TextInput => self.render_text_input(question, current_answer),
                        QuestionType::TrueFalse => self.render_true_false(question, current_answer),
                        _ => self.render_options(question, current_answer),
                    } 
                }
            </div>
        }
    }

    fn render_text_input(&self, question: &Question, current_answer: Option<&UserAnswer>) -> Html {
        let text = match current_answer {
            Some(UserAnswer::TextInput(_, t)) => t.clone(),
            _ => String::new(),
        };

        let question_id = question.id;
        let question_clone = question.clone();
        html! {
            <div class="answer-input mt-2">
                <textarea class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-12 py-5" aria-label="With textarea" 
                    style="min-height: 100px;"
                    placeholder="Texto de la respuesta"
                    rows="4"
                    cols="50"
                    value={ text }
                    oninput={ self.link.callback(move |e: InputData| ResponderMsg::SetTextAnswer(question_clone.clone(), question_id, e.value)) }>
                </textarea>
            </div>
        }
    }

    fn render_true_false(&self, question: &Question, current_answer: Option<&UserAnswer>) -> Html {
        let is_true = matches!(current_answer, Some(UserAnswer::TrueFalse(_,_, true)));
        let is_false = matches!(current_answer, Some(UserAnswer::TrueFalse(_, _, false)));
        let question_id = question.id;
        let question_clone_true = question.clone();
        let question_clone_false = question.clone();

        let (true_option_id, false_option_id) = match question.options.as_slice() {
            [true_opt, false_opt] => (true_opt.id, false_opt.id),
            _ => (Uuid::default(), Uuid::default()), // Fallback if there are not exactly two
        };
    
        html! {
            <div class="options mt-2">
                <div class="col">
                    <div class="col">
                        <div class="form-check">
                            <input 
                                class="form-check-input" 
                                type="radio" 
                                name={ format!("truefalse-{}", question.id) }
                                checked={ is_true }
                                oninput={ self.link.callback(move |_| {
                                    ResponderMsg::SetTrueFalse(question_clone_true.clone(), question_id, true_option_id, true)
                                }) }
                            />
                            <label class="form-check-label">
                                { "Verdadero" }
                            </label>
                        </div>
                    </div>
                    <div class="col">
                        <div class="form-check">
                            <input 
                                class="form-check-input" 
                                type="radio" 
                                name={ format!("truefalse-{}", question.id) }
                                checked={ is_false }
                                onclick={ self.link.callback(move |_| {
                                    ResponderMsg::SetTrueFalse(question_clone_false.clone(), question_id, false_option_id, false)
                                }) }
                            />
                            <label class="form-check-label">
                                { "Falso" }
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    fn render_options(&self, question: &Question, current_answer: Option<&UserAnswer>) -> Html {
        html! {
            <div class="options mt-2">
                { for question.options.iter().map(|option| self.render_option(question, option, current_answer)) }
            </div>
        }
    }

    fn render_option(
        &self, 
        question: &Question, 
        option: &AnswerOption, 
        current_answer: Option<&UserAnswer>
    ) -> Html {
        let is_checked = match (current_answer, &question.question_type) {
            (Some(UserAnswer::SingleChoice(_, id)), QuestionType::SingleChoice) => 
                id == &option.id,
            (Some(UserAnswer::MultipleChoice(_, ids)), QuestionType::MultipleChoice) => 
                ids.contains(&option.id),
            _ => false,
        };

        let input_type = match question.question_type {
            QuestionType::SingleChoice => "radio",
            QuestionType::MultipleChoice => "checkbox",
            _ => "radio", // Por defecto, aunque no debería ocurrir
        };

        let question_id = question.id;
        let option_id = option.id;
        let question_clone = question.clone();

        let callback = match question.question_type {
            QuestionType::SingleChoice => self.link.callback(move |_| {
                ResponderMsg::SelectSingleChoice(question_clone.clone(), question_id, option_id)
            }),
            QuestionType::MultipleChoice => self.link.callback(move |_| {
                ResponderMsg::ToggleMultipleChoice(question_clone.clone(), question_id, option_id)
            }),
            _ => Callback::noop(), // No debería ocurrir
        };

        html! {
            <div class="form-check mb-2">
                <input 
                    class="form-check-input" 
                    type={ input_type }
                    name={ format!("question-{}", question.id) }
                    checked={ is_checked  }
                    onclick={ callback }
                />
                <label class="form-check-label">
                    { &option.text }
                </label>
            </div>
        }
    }
}