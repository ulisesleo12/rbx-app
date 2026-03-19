use log::info;
use uuid::Uuid;
use yew::prelude::*;
use chrono::{Local, NaiveDateTime};
use code_location::code_location;
use crate::quizviewer::QuizViewer;
use crate::quizcreator::QuizCreator;


use roboxmaker_models::quiz_model::{self, *};
use roboxmaker_types::types::{GroupId, SchoolId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};



// Type listed for different types of questio
#[derive(Clone, Debug, PartialEq)]
pub enum QuizMode {
    Create,
    Edit,
}

// Type listed for different types of question
#[derive(Clone, Debug, PartialEq)]
pub enum QuizzTabOptions {
    Editor,
    View,
}

// Type for an answer option
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AnswerOption {
    pub id: Uuid,
    pub text: String,
    pub is_correct: bool,
}

// Type for a question
#[derive(Clone, Debug, PartialEq)]
pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub question_type: QuestionType,
    pub options: Vec<AnswerOption>,
    pub points: u32,
}

// Type listed for different types of question
#[derive(Clone, Debug, PartialEq)]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    TrueFalse,
    TextInput,
}

impl Default for QuestionType {
    fn default() -> Self {
        QuestionType::SingleChoice
    }
}

// Type for a author quiz response
#[derive(Clone, Debug, PartialEq)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub full_name: String,
    pub pic_path: String,
}

// Type for a quiz response
#[derive(Clone, Debug, PartialEq, Default)]
pub struct QuizResponses {
    pub quiz_response_id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub status: String,
    pub started_at: NaiveDateTime,
    pub completed_at: NaiveDateTime,
}

// Type for a question section
#[derive(Clone, Debug, PartialEq)]
pub struct QuestionSection {
    pub id: Uuid,
    pub title: String,
    pub description: String,  
    pub observation: String,  
    pub questions: Vec<Question>,
}

// Type for the full quiz
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub create_at: NaiveDateTime,
    pub state: String,
    pub author_id: Option<Uuid>,
    pub sections: Vec<QuestionSection>,
    pub quiz_responses: Vec<QuizResponses>,
    pub author_profile: Option<UserProfile>,
    pub is_evaluation: bool,
}

pub struct AppState {
    pub current_quiz: Quiz,
    pub current_section_index: usize,
    pub current_question_index: usize,
    pub quiz_created: bool,
}

// User response
#[derive(Clone, Debug)]
pub enum UserAnswer {
    SingleChoice(Question ,Uuid), // Selected option ID
    MultipleChoice(Question, Vec<Uuid>), // IDS of the selected options
    TrueFalse(Question, Uuid, bool),
    TextInput(Question, String),
}

pub struct QuizApp {
    state: AppState,
    link: ComponentLink<Self>,
    props: QuizAppProperties,
    graphql_task: Option<GraphQLTask>,
    new_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    view_tab: QuizzTabOptions,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct QuizAppProperties {
    pub quiz: Option<Quiz>,
    pub quiz_mode: QuizMode,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub quiz_id: Option<Uuid>,
    pub on_quiz_responder_page: Option<Callback<MouseEvent>>,
    pub is_evaluation: bool,
}

// Messages to update the state
#[derive(Debug)]
pub enum Msg {
    ViewTab(QuizzTabOptions),
    UpdateQuiz(Quiz),
    // AddAnswerOption(String, String, AnswerOption), // section_id, question_id, option
    // UpdateAnswerOption(String, String, AnswerOption), // section_id, question_id, option
    // DeleteAnswerOption(String, String, String), // section_id, question_id, option_id
    // SelectAnswer(UserAnswer),
    // NextQuestion,
    // PreviousQuestion,
    // SubmitQuiz,
    SaveQuiz,
    SaveQuizResponse(Option<quiz_model::update_quiz::ResponseData>),
    UpdateSetSection(QuestionSection),
    NewSectionResponse(Option<quiz_model::update_inset_section::ResponseData>),
    CreateNewQuestion(Uuid, Question),
    NewQuestionResponse(Option<quiz_model::upset_question_answer_options::ResponseData>),
    DeleteSection(Uuid),
    DelSectionResponse(Option<quiz_model::delete_question_section_by_id::ResponseData>),
    DeleteQuestion(Uuid, Uuid), // section_id, question_id
    DelQuestionResponse(Option<quiz_model::delete_question_by_id::ResponseData>),
    DeleteOption(Uuid, Uuid), // question_id, answer_id
    DelOptionResponse(Option<quiz_model::delete_option_by_id::ResponseData>),
    ToggleEvaluation,
}

impl Component for QuizApp {
    type Message = Msg;
    type Properties = QuizAppProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let quiz_id = if props.quiz_id.is_some() {
            props.quiz_id.unwrap()
        } else {
            Uuid::new_v4()
        };

        let current_quiz = if props.quiz.is_some() {
            props.quiz.clone().unwrap()
        } else {
            Quiz {
                id: quiz_id,
                title: String::new(),
                description: String::new(),
                create_at: chrono::Local::now().naive_local(),
                state: String::new(),
                sections: Vec::new(),
                quiz_responses: Vec::new(),
                author_id: None,
                author_profile: None,
                is_evaluation: props.is_evaluation,
            }
        };

        QuizApp {
            state: AppState {
                current_quiz,
                current_section_index: 0,
                current_question_index: 0,
                quiz_created: false,
                // is_evaluation: true,
            },
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            new_task: None,
            delete_task: None,
            view_tab: QuizzTabOptions::Editor,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizApp: {:?}", msg);
        match msg {
            Msg::ViewTab(new_tab) => {
                self.view_tab = new_tab;
                true
            }
            Msg::UpdateQuiz(quiz) => {
                self.state.current_quiz.update_quiz(quiz);
                true
            }
            Msg::ToggleEvaluation => {
                if let Some(quiz) = &self.props.quiz {
                    let corrects = quiz.sections.iter().any(|sect| sect.questions.iter().any(|q| q.options.iter().any(|opt| opt.is_correct ) && q.points > 0));
                    // let questions = quiz.sections.iter().map(|s| s.questions.len()).sum::<usize>();

                    if quiz.is_evaluation && !corrects || !quiz.is_evaluation {
                        self.state.current_quiz.is_evaluation = !self.state.current_quiz.is_evaluation; 
                    }
                }
                true
            }
            Msg::SaveQuiz => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = update_quiz::Variables { 
                        quiz_id: self.state.current_quiz.id,
                        title: self.state.current_quiz.title.clone(),
                        description: Some(self.state.current_quiz.description.clone()),
                        state: String::from("CREATED"),
                        updated_at: Local::now().naive_local(),
                        is_evaluation: self.state.current_quiz.is_evaluation
                    };

                    let task = UpdateQuiz::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Msg::SaveQuizResponse(response)
                        },
                    );
                    self.new_task = Some(task);
                }
                false
            }
            Msg::SaveQuizResponse(response) => {
                if response.is_some() {
                    info!("SaveQuizResponse: {:?}", response.unwrap());

                    if self.props.quiz_mode == QuizMode::Create {
                        for section in self.state.current_quiz.sections.clone() {
                            self.link.send_message(Msg::UpdateSetSection(section.clone()));
                            
                            let section_id = section.id;
                            for question in section.questions.clone() {
                                self.link.send_message(Msg::CreateNewQuestion(section_id, question));
                            }
                        }
                    }
                } else {
                    log::error!("Failed to Save Quiz")
                }
                true
            }
            Msg::UpdateSetSection(section) => {
                info!("Current sections: {:?}", self.state.current_quiz.sections);
                if let Some(graphql_task) = self.graphql_task.as_mut() {        
                    let vars = update_inset_section::Variables { 
                        title: section.title,
                        description: Some(section.description),
                        observation: Some(section.observation),
                        quiz_id: self.state.current_quiz.id,
                        section_id: section.id,
                        section_order: self.state.current_quiz.sections.iter().position(|sec| sec.id == section.id).and_then(|idx| Some(idx as i64)),
                    };
    
                    let task = UpdateInsetSection::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Msg::NewSectionResponse(response)
                        },
                    );
                    self.new_task = Some(task);
                }
                true
            }
            Msg::NewSectionResponse(response) => {
                if response.is_some() {
                    info!("NewSectionResponse: {:?}", response.unwrap())
                } else {
                    log::error!("Failed to UpSet Section")
                }
                true
            }
            Msg::CreateNewQuestion(section_id, question) => {
                if let Some(section) = self.state.current_quiz.get_section(section_id) {
                    info!("Section questions {}: {:?}", section_id, section.questions);
                    
                    if let Some(graphql_task) = self.graphql_task.as_mut() {    
                        let question_order = section
                            .questions
                            .iter()
                            .position(|q| q.id == question.id)
                            .and_then(|idx| Some(idx as i64)); 
                        
                        let answer_options = question.options
                            .iter()
                            .enumerate()
                            .map(|(k, option)| {
                                upset_question_answer_options::AnswerOptionsInsertInput {
                                    option: Some(option.text.clone()),
                                    option_order: Some(k as i64),
                                    is_correct: Some(option.is_correct),
                                    id: Some(option.id),
                                    question_id: Some(question.id),
                                    question: None,
                                }
                            })
                            .collect();

                        let maybe_correct_option = question.options.iter().any(|options| options.is_correct);

                        let points = if maybe_correct_option || question.question_type == QuestionType::TextInput { Some(question.points as i64) } else { None };

                        let vars = upset_question_answer_options::Variables { 
                            question_id: question.id,
                            question: question.text,
                            points,
                            section_id,
                            question_order,
                            question_type: match question.question_type {
                                QuestionType::SingleChoice => upset_question_answer_options::RoboxQuestionTypeEnum::SingleChoice,
                                QuestionType::MultipleChoice => upset_question_answer_options::RoboxQuestionTypeEnum::MultipleChoice,
                                QuestionType::TrueFalse => upset_question_answer_options::RoboxQuestionTypeEnum::TrueFalse,
                                QuestionType::TextInput => upset_question_answer_options::RoboxQuestionTypeEnum::TextInput,
                            },
                            answer_options
                        };
        
                        let task = UpsetQuestionAnswerOptions::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                Msg::NewQuestionResponse(response)
                            },
                        );
                        self.new_task = Some(task);    
                    }
                }
                true
            }
            Msg::NewQuestionResponse(response) => {
                if response.is_some() {
                    info!("NewQuestionResponse: {:?}", response.unwrap())
                } else {
                    log::error!("Failed to UpSet Question")
                }
                true
            }
            Msg::DeleteSection(section_id) => {
                info!("Section deleted: {}", section_id);
                // self.state.current_quiz.remove_section(section_id);
                if let Some(graphql_task) = self.graphql_task.as_mut() {        
                    let vars = delete_question_section_by_id::Variables {
                        quiz_id: self.state.current_quiz.id,
                        section_id,
                    };
    
                    let task = DeleteQuestionSectionById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Msg::DelSectionResponse(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
                true
            }
            Msg::DelSectionResponse(response) => {
                if response.is_some() {
                    info!("DelSectionResponse: {:?}", response.unwrap())
                } else {
                    log::error!("Failed to Delete Section")
                }
                true
            }
            Msg::DeleteQuestion(_section_id, question_id) => {
                info!("Question deleted: {}", question_id);
                // self.state.current_quiz.remove_question(section_id, question_id)
                if let Some(graphql_task) = self.graphql_task.as_mut() {        
                    let vars = delete_question_by_id::Variables {
                        // section_id,
                        question_id,
                    };
    
                    let task = DeleteQuestionById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Msg::DelQuestionResponse(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
                true
            }
            Msg::DelQuestionResponse(response) => {
                if response.is_some() {
                    info!("DelQuestionResponse: {:?}", response.unwrap())
                } else {
                    log::error!("Failed to Delete Question")
                }
                true
            }
            Msg::DeleteOption(question_id, answer_id) => {
                info!("Question deleted: {}", question_id);
                // self.state.current_quiz.remove_question(section_id, question_id)
                if let Some(graphql_task) = self.graphql_task.as_mut() {        
                    let vars = delete_option_by_id::Variables {
                        question_id,
                        answer_id,
                    };
    
                    let task = DeleteOptionById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Msg::DelOptionResponse(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
                true
            }
            Msg::DelOptionResponse(response) => {
                if response.is_some() {
                    info!("DelOptionResponse: {:?}", response.unwrap())
                } else {
                    log::error!("Failed to Delete Option")
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let tab_class = |flag: bool | match flag {
            true => "nav-link active is-active-tab",
            false => "nav-link is-no-active-tab",
        };

        let is_disable_btn = if self.props.quiz.is_some() {
            let quiz = self.props.quiz.clone().unwrap();
            !(quiz.title != self.state.current_quiz.title 
                || quiz.description != self.state.current_quiz.description
                || quiz.is_evaluation != self.state.current_quiz.is_evaluation
            ) 
            || self.state.current_quiz.is_evaluation 
                && self.state.current_quiz.sections.iter().any(|sec| 
                    !sec.questions.iter().any(|q| q.options.iter().any(|opt| opt.is_correct) && q.points > 0)
            )
        } else {
            false
        };
        html! {
            <div class="quiz-app">
                <ul class="nav nav-tabs nav-fill mt-6">
                    <li class="nav-item">
                        <a class={ tab_class(self.view_tab == QuizzTabOptions::Editor) } 
                            onclick={ self.link.callback(|_| Msg::ViewTab(QuizzTabOptions::Editor)) }>
                            { "Editor" }
                        </a>
                    </li>
                    <li class="nav-item">
                        <a class={ tab_class(self.view_tab == QuizzTabOptions::View) } 
                            onclick={ self.link.callback(|_| Msg::ViewTab(QuizzTabOptions::View)) }>
                            { "Ver" }
                        </a>
                    </li>
                </ul>
                {
                    match self.view_tab {
                        QuizzTabOptions::Editor => {
                            html! {
                                <QuizCreator 
                                    quiz={ self.state.current_quiz.clone() } 
                                    quiz_mode={ self.props.quiz_mode.clone() }
                                    on_upset_section={ self.link.callback(|section| Msg::UpdateSetSection(section)) }
                                    on_upset_question={ self.link.callback(|(sid, q)| Msg::CreateNewQuestion(sid, q)) }
                                    on_update_quiz={ self.link.callback(|quiz| Msg::UpdateQuiz(quiz)) }
                                    on_delete_section={ self.link.callback(|section_id| Msg::DeleteSection(section_id)) }
                                    on_delete_question={ self.link.callback(|(section_id, question_id)| Msg::DeleteQuestion(section_id, question_id)) }
                                    on_delete_option={ self.link.callback(|(question_id, answer_id)| Msg::DeleteOption(question_id, answer_id)) }
                                    on_is_evaluation={ self.link.callback(|_| Msg::ToggleEvaluation) }
                                    is_evaluation={self.state.current_quiz.is_evaluation}
                                />
                            }
                        }
                        QuizzTabOptions::View => {
                            html! { 
                                    <QuizViewer 
                                    quiz={ self.state.current_quiz.clone() } 
                                /> 
                            }
                        }
                    }
                } 
                {
                    if self.props.quiz_mode == QuizMode::Create {
                        html! {
                            <div class="d-flex justify-content-center mt-6">
                                <a class="btn button-saved-post d-flex align-items-center justify-content-center" style="min-width: 230px;" onclick={ self.link.callback(|_| Msg::SaveQuiz) }>
                                    <span class="text-white noir-bold is-size-16 lh-20">{ "Guardar Cuestionario" }</span>
                                </a>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="d-flex justify-content-center mt-6">
                                <button class="btn button-saved-post d-flex align-items-center justify-content-center mx-4" 
                                    disabled={ is_disable_btn }
                                    onclick={ self.link.callback(move |_| Msg::SaveQuiz) }>
                                    <span class="text-white noir-bold is-size-16 lh-20">{ "Actualizar" }</span>
                                </button>
                                {
                                    if self.props.on_quiz_responder_page.is_some() {
                                        html! {
                                            <button class="btn button-saved-eraser bg-gray-strong d-flex align-items-center justify-content-center mx-4" 
                                                onclick={ self.props.on_quiz_responder_page.clone().unwrap() }
                                                >
                                                <span class="text-white noir-bold is-size-16 lh-20">{ "Cancelar" }</span>
                                            </button>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        }
                    }
                }
            </div>
        }
    }
}