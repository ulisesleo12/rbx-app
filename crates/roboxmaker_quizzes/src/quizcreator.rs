use log::info;
use uuid::Uuid;
use yew::prelude::*;
use crate::quiz::{AnswerOption, Question, QuestionSection, QuestionType, Quiz, QuizMode};


#[derive(Clone, Debug, PartialEq)]
pub enum QuizCreatorMode {
    None,
    NewSection,
    EditSection,
    NewQuestion(QuestionSection),
    EditQuestion(QuestionSection),
}

pub struct QuizCreator {
    pub props: QuizCreatorProps,
    pub link: ComponentLink<Self>,
    pub upset_section: QuestionSection,
    pub upset_question: Question,
    pub upset_option_text: String,
    pub quiz_edit_mode: QuizCreatorMode,
    pub is_q_expanded: Option<Uuid>,
    pub is_o_expanded: Option<AnswerOption>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct QuizCreatorProps {
    pub quiz: Quiz,
    pub quiz_mode: QuizMode,
    pub on_upset_section: Callback<QuestionSection>,
    pub on_upset_question: Callback<(Uuid, Question)>,
    pub on_update_quiz: Callback<Quiz>,
    pub on_delete_section: Callback<Uuid>,
    pub on_delete_question: Callback<(Uuid, Uuid)>,
    pub on_delete_option: Callback<(Uuid, Uuid)>,
    pub on_is_evaluation: Callback<MouseEvent>,
    pub is_evaluation: bool,
}

#[derive(Debug)]
pub enum CreatorMsg {
    QuizCreatorActions(QuizCreatorMode),

    UpdateQuizTitle(String),
    UpdateQuizDescription(String),
    
    UpdateSectionTitle(String),
    UpdateSectionDescription(String),
    UpdateSectionObservation(String),

    AddSection(Option<QuestionSection>),
    EditSection(Option<QuestionSection>),
    DeleteSection(Uuid),
    UpdateSection,

    ShowQuestions(Option<Uuid>),

    UpdateQuestionText(String),
    UpdateQuestionType(Uuid, QuestionType, QuestionType),
    UpdateQuestionPoints(Uuid, bool),

    
    AddQuestion(Option<Uuid>),
    EditQuestion(Option<QuestionSection>, Option<Question>),
    DeleteQuestion(Uuid, Uuid),
    UpdateQuestion(Uuid, Question),
    
    UpdateOptionText(String),
    ToggleOptionCorrect(Uuid),
    UpdateQuestionOptions(Vec<AnswerOption>),

    AddOption(Uuid, Uuid),
    EditOption(Option<AnswerOption>),
    DeleteOption(Uuid, Uuid),
    UpdateOption(Uuid),
}

impl Component for QuizCreator {
    type Message = CreatorMsg;
    type Properties = QuizCreatorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        info!("QUIZ: {:?}", props.quiz);

        let quiz_edit_mode = if props.quiz_mode == QuizMode::Create {
            QuizCreatorMode::NewSection
        } else {
            QuizCreatorMode::None
        };

        QuizCreator {
            props,
            link,
            upset_section: QuestionSection::default(),
            upset_question: Question::default(),
            upset_option_text: String::new(),
            quiz_edit_mode,
            is_q_expanded: None,
            is_o_expanded: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("msg {:?}", msg);
        match msg {
            CreatorMsg::QuizCreatorActions(quiz_creator_mode) => {
                self.quiz_edit_mode = quiz_creator_mode;
                true
            }
            CreatorMsg::UpdateQuizTitle(title) => {
                let mut quiz = self.props.quiz.clone();
                quiz.title = title;
                self.props.on_update_quiz.emit(quiz);
                true
            }
            CreatorMsg::UpdateQuizDescription(desc) => {
                let mut quiz = self.props.quiz.clone();
                quiz.description = desc;
                self.props.on_update_quiz.emit(quiz);
                true
            }
            CreatorMsg::UpdateSectionTitle(title) => {
                self.upset_section.title = title;
                true
            }
            CreatorMsg::UpdateSectionDescription(desc) => {
                self.upset_section.description = desc;
                true
            }
            CreatorMsg::UpdateSectionObservation(obs) => {
                self.upset_section.observation = obs;
                true
            }
            CreatorMsg::AddSection(opt_q_section) => {
                if let Some(q_section) = opt_q_section {
                    if !q_section.title.is_empty() {
                        let section = q_section.clone();
                        let mut update_quiz = self.props.quiz.clone();
                        update_quiz.add_section(section);
                        self.props.on_update_quiz.emit(update_quiz);

                        if self.props.quiz_mode == QuizMode::Edit {
                            self.props.on_upset_section.emit(q_section);
                        }
    
                        // Reset for new section
                        self.upset_section = QuestionSection::default();
                        self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                        
                        info!("QUIZ: {:?}", self.props.quiz.sections);
                        true
                    } else {
                        false
                    }
                } else {
                    self.upset_section = QuestionSection::default();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                    true
                }
            }
            CreatorMsg::EditSection(section) => {
                if let Some(section) = section {
                    self.upset_section = section.clone();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::EditSection));
                } else {
                    self.upset_section = QuestionSection::default();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                }
                true
            }
            CreatorMsg::DeleteSection(section_id) => {
                let mut update_quiz = self.props.quiz.clone();
                update_quiz.remove_section(section_id);
                self.props.on_update_quiz.emit(update_quiz);

                if self.props.quiz_mode == QuizMode::Edit {
                    self.props.on_delete_section.emit(section_id);
                }

                self.link.send_message(CreatorMsg::EditSection(None));
                true
            }
            CreatorMsg::UpdateSection => {
                let section = self.upset_section.clone();
                let mut update_quiz = self.props.quiz.clone();
                update_quiz.update_section(section.clone());
                self.props.on_update_quiz.emit(update_quiz);

                if self.props.quiz_mode == QuizMode::Edit {
                    self.props.on_upset_section.emit(section);
                }

                // Reset for new section
                self.upset_section = QuestionSection::default();
                self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                true
            }
            CreatorMsg::ShowQuestions(is_q_expanded) => {
                self.is_q_expanded = is_q_expanded;
                true
            }
            CreatorMsg::UpdateQuestionText(text) => {
                self.upset_question.text = text;
                true
            }
            CreatorMsg::UpdateQuestionPoints(_section_id, operation) => {
                if operation {
                    self.upset_question.points += 1;
                } else {
                    if self.upset_question.points > 1 {
                        self.upset_question.points -= 1;
                    }
                }
                true
            }
            CreatorMsg::UpdateQuestionType(section_id, new_q_type, prev_q_type) => {
                let preserve = matches!((prev_q_type.clone(), new_q_type.clone()),
                    (QuestionType::MultipleChoice, QuestionType::SingleChoice)
                    | (QuestionType::SingleChoice, QuestionType::MultipleChoice)
                );
            
                let question_id = self.upset_question.id;

                if prev_q_type == QuestionType::MultipleChoice && new_q_type == QuestionType::SingleChoice {
                    let mut found_correct = false; 
                    for opt in self.upset_question.options.iter_mut() {
                        if opt.is_correct {
                            if !found_correct {
                                found_correct = true;
                            } else {
                                opt.is_correct = false;
                            }
                        }
                    }
                }

                if preserve {
                    self.upset_question.question_type = new_q_type;
                } else {
                    self.upset_question.question_type = new_q_type.clone();
                    
                    let mut update_quiz = self.props.quiz.clone();
                    update_quiz.clear_options_from_question(section_id, question_id);
                    self.props.on_update_quiz.emit(update_quiz);

                    if self.props.quiz_mode == QuizMode::Edit {
                        for option in &self.upset_question.options {
                            self.props.on_delete_option.emit((question_id, option.id));
                        }
                    }            
                    self.upset_question.options.clear(); // Clean if not preserved

                    if self.upset_question.question_type == QuestionType::TrueFalse {
                        self.upset_question.options = vec![
                            AnswerOption { id: Uuid::new_v4(), text: "Verdadero".to_string(), is_correct: false },
                            AnswerOption { id: Uuid::new_v4(), text: "Falso".to_string(), is_correct: false },
                        ];
                    }

                    if new_q_type == QuestionType::TextInput {
                        self.upset_question.points = 1;
                    }
                }
                true
            }
            CreatorMsg::UpdateOptionText(text) => {
                self.upset_option_text = text;
                true
            }
            CreatorMsg::ToggleOptionCorrect(id) => {
                let validate = self.upset_question.options.iter().any(|opt| opt.id == id && opt.is_correct );

                if !validate {
                    let maybe_before_option_correct = self.upset_question.options.iter().any(|option| option.is_correct);

                    if self.upset_question.question_type == QuestionType::TrueFalse && maybe_before_option_correct {
                        for option in self.upset_question.options.iter_mut() {
                            if option.id != id && option.is_correct {
                                option.is_correct = false;
                            }
                        }
                    }
                    let option_index = self.upset_question.options.iter().position(|opt| opt.id == id);

                    if let Some(index) = option_index {
                        let should_unset_others = self.upset_question.question_type == QuestionType::SingleChoice
                            && !self.upset_question.options[index].is_correct;

                        self.upset_question.options[index].is_correct = !self.upset_question.options[index].is_correct;

                        if should_unset_others {
                            for (i, opt) in self.upset_question.options.iter_mut().enumerate() {
                                if i != index {
                                    opt.is_correct = false;
                                }
                            }
                        }
                    }

                    let maybe_after_option_correct = self.upset_question.options.iter().any(|option| option.is_correct);
                    if !maybe_before_option_correct && maybe_after_option_correct && self.props.is_evaluation {
                        self.upset_question.points = 1;
                    }
                }
                true
            }
            CreatorMsg::AddQuestion(opt_section_id) => {
                if let Some(section_id) = opt_section_id {
                    if !self.upset_question.text.is_empty() && (!self.upset_question.options.is_empty() || self.upset_question.question_type.clone() == QuestionType::TextInput) {
                        if !self.upset_question.options.iter().any(|item| item.is_correct) 
                            && self.upset_question.question_type.clone() != QuestionType::TextInput {
                            self.upset_question.points = 0;
                        }
                        let question = self.upset_question.clone();
                        
                        let mut update_quiz = self.props.quiz.clone();
                        update_quiz.add_question(section_id, question.clone());
                        self.props.on_update_quiz.emit(update_quiz);

                        if self.props.quiz_mode == QuizMode::Edit {
                            self.props.on_upset_question.emit((section_id, question));
                        }
                        
                        // Reset for new question
                        self.upset_question = Question::default();
                        self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                        true
                    } else {
                        false
                    }
                } else {
                    self.upset_question = Question::default();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                    true
                }
            }
            CreatorMsg::EditQuestion(opt_section, opt_question ) => {
                if let (Some(q_section), Some(question)) = (&opt_section, &opt_question) {
                    self.upset_question = question.clone();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::EditQuestion(q_section.clone())))
                } else {
                    self.upset_question = Question::default();
                    self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None))
                }
                true
            }
            CreatorMsg::DeleteQuestion(section_id, question_id) => {
                let mut update_quiz = self.props.quiz.clone();
                update_quiz.remove_question(section_id, question_id);
                self.props.on_update_quiz.emit(update_quiz);

                if self.props.quiz_mode == QuizMode::Edit {
                    self.props.on_delete_question.emit((section_id, question_id));
                }

                self.link.send_message(CreatorMsg::EditQuestion(None, None));
                true
            }
            CreatorMsg::UpdateQuestion(section_id, mut question) => {
                let mut update_quiz = self.props.quiz.clone();
                if !question.options.iter().any(|option| option.is_correct) 
                    && self.upset_question.question_type.clone() != QuestionType::TextInput {
                    question.points = 0;
                }
                
                update_quiz.update_question(section_id, question.clone());
                self.props.on_update_quiz.emit(update_quiz);


                if self.props.quiz_mode == QuizMode::Edit {
                    self.props.on_upset_question.emit((section_id, question));
                }

                self.upset_question = Question::default();
                self.link.send_message(CreatorMsg::QuizCreatorActions(QuizCreatorMode::None));
                true
            }
            CreatorMsg::UpdateQuestionOptions(options) => {
                self.upset_question.options = options;

                info!("QuizApp: {:?}", self.upset_question.options);
                info!("QuizApp: {:?}", self.props.quiz.sections);
                true
            }
            CreatorMsg::AddOption(section_id, question_id) => {
                if !self.upset_option_text.is_empty() {
                    let option = AnswerOption::new(self.upset_option_text.clone());
                    self.upset_question.options.push(option.clone());

                    let mut update_quiz = self.props.quiz.clone();
                    update_quiz.add_option(section_id, question_id, option);
                    self.props.on_update_quiz.emit(update_quiz);

                    // if self.props.quiz_mode == QuizMode::Edit {
                    //     self.props.on_upset_question.emit((section_id, self.upset_question.clone()));
                    // }

                    self.upset_option_text.clear();
                    true
                } else {
                    false
                }
            }
            CreatorMsg::EditOption(is_o_expanded) => {
                self.is_o_expanded = is_o_expanded;
                if let Some(option) = &self.is_o_expanded {
                    self.upset_option_text = option.text.clone();
                } else {
                    self.upset_option_text.clear();
                }
                true
            }
            CreatorMsg::DeleteOption(section_id, option_id) => {
                self.upset_question.options.retain(|opt| opt.id != option_id);
                info!("Delete option: {} - {:?}", option_id, self.upset_question.options);

                if self.props.quiz_mode == QuizMode::Edit {
                    let question_id = self.upset_question.id; 
                    let mut update_quiz = self.props.quiz.clone();
                    update_quiz.remove_option(section_id, question_id, option_id);
                    self.props.on_update_quiz.emit(update_quiz);
    
                    self.props.on_delete_option.emit((question_id, option_id));
                }
                true
            }
            CreatorMsg::UpdateOption(section_id) => {
                if let Some(a_option) = &self.is_o_expanded {
                    for options in self.upset_question.options.iter_mut() {
                        if options.id == a_option.id {
                            options.text = self.upset_option_text.clone()
                        }
                    }

                    if self.props.quiz_mode == QuizMode::Edit {
                        self.props.on_upset_question.emit((section_id, self.upset_question.clone()));
                    }
                    self.upset_option_text.clear();
                    self.link.send_message(CreatorMsg::EditOption(None))
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
        let quiz_mode_none = self.quiz_edit_mode == QuizCreatorMode::None;
        html! {
            <div class="quiz-creator pt-5">
                <div class="card quiz-meta mb-4">
                    <div class="card-body">
                        <div class="d-flex justify-content-between">
                            <h2 class="text-primary-blue-dark text-camelcase noir-bold is-size-36 lh-43 mb-3">{"Configuración del Cuestionario"}</h2>
                            <div class="form-check form-switch">
                                <input class="form-check-input" type="checkbox" id="flexSwitchCheckDefault" 
                                    checked={self.props.is_evaluation}  
                                    onclick={self.props.on_is_evaluation.clone()} disabled={ self.props.is_evaluation && self.props.quiz.sections.iter().any(|sec| sec.questions.iter().any(|q| q.options.iter().any(|opt| opt.is_correct) && q.points > 0 ))} />
                                <label class="form-check-label" for="flexSwitchCheckDefault">{"Evaluación"}</label>
                            </div>
                        </div>
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Título*"}</label>
                            <input type="text" class="input form-control input-style-universal" id="exampleFormControlInput1" placeholder="Título del cuestionario"
                                disabled={ !quiz_mode_none && !self.props.quiz.sections.is_empty() }
                                value={ self.props.quiz.title.clone() }
                                oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateQuizTitle(e.value)) } 
                                />
                        </div>
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Descripción"}</label>
                            // <input type="text" class="form-control input-style-universal" id="exampleFormControlInput1" placeholder="Descripción"
                            //     disabled={ !(self.quiz_edit_mode == QuizCreatorMode::None) }
                            //     value={ self.props.quiz.description.clone() }
                            //     oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateQuizDescription(e.value)) }
                            //     />
                            <textarea class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-12 py-5" aria-label="With textarea" 
                                style="min-height: 120px;"
                                placeholder="Texto de la pregunta"
                                rows="3"
                                disabled={ !quiz_mode_none && !self.props.quiz.sections.is_empty() }
                                value={ self.props.quiz.description.clone() }
                                oninput=self.link.callback(|e: InputData| CreatorMsg::UpdateQuizDescription(e.value))></textarea>
                                // <div class="mb-6" style="border: 1px solid #C8C1CD; border-radius: 10px;">
                                    // <ckeditor::CKEditor user_profile=self.props.user_profile.clone()
                                    //         content=self.content.clone()
                                    //         upload_url=upload_url
                                    //         on_data=on_data />
                            // </div>
                        </div>
                    </div>
                </div>

                <h4 class="text-blue-two text-camelcase noir-medium is-size-32 lh-43 mb-3"><u>{"Secciones del Cuestionario"}</u></h4>
                
                <div class="sections-list">
                    { for self.props.quiz.sections.iter().map(|section| self.render_section_item(section)) }
                    {
                        if !self.props.quiz.sections.is_empty() && self.quiz_edit_mode == QuizCreatorMode::None {
                            html! {
                                <div class="alert bg-light-sea-green noir-medium text-white" role="alert">
                                    <p class="mb-0">{"Edita una sección o agrega preguntas a la sección"}</p>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                {
                    match &self.quiz_edit_mode {
                        QuizCreatorMode::None => html! {
                            <div class="d-flex justify-content-center mb-5 mt-6">
                                <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4" style="width: 230px;"
                                    onclick={ self.link.callback(|_| CreatorMsg::QuizCreatorActions(QuizCreatorMode::NewSection)) }>
                                    <span class="text-white noir-bold is-size-16 lh-20">{ "Agregar nueva sección" }</span>
                                </button>
                            </div>
                        },
                        QuizCreatorMode::NewSection => html! {
                            self.render_upset_section_item()
                        },
                        QuizCreatorMode::EditSection => html! {
                            self.edit_section_item()
                        },
                        QuizCreatorMode::NewQuestion(section) => html! {
                            <>
                                { self.render_upset_question_item(&section) }
                                {
                                    if self.props.quiz.sections.is_empty() {
                                        html! {
                                            <div class="alert bg-light-sea-green noir-medium text-white" role="alert">
                                                <p class="mb-0">{"Crea una sección para comenzar el cuestionario"}</p>
                                            </div>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </>
                        },
                        QuizCreatorMode::EditQuestion(section) => html! {
                            self.edit_question_item(&section)
                        },
                    }
                }
            </div>
        }
    }
}