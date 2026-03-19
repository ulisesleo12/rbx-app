use yew::prelude::*;
use uuid::Uuid;
use crate::{quiz::{AnswerOption, Question, QuestionSection, QuestionType}, quizcreator::{CreatorMsg, QuizCreator, QuizCreatorMode}};

impl QuizCreator {
    fn section_validation(&self, section: &QuestionSection) -> bool {
        let validation = section.questions.iter()
            .any(|question| 
                !question.options.iter().any(|opt| opt.is_correct)
                && question.question_type != QuestionType::TextInput
                || question.points == 0 
            );

        validation
    }
    pub fn render_section_item(&self, section: &QuestionSection) -> Html {
        let section_clone = section.clone();
        let upset_question = section.clone();
        let section_id = section.id.clone();

        let is_selected = self.quiz_edit_mode == QuizCreatorMode::EditSection;
        let show_upset_section = self.quiz_edit_mode == QuizCreatorMode::NewSection;
        let show_upset_question = self.quiz_edit_mode == QuizCreatorMode::NewQuestion(upset_question.clone());
        let is_expanded = self.is_q_expanded == Some(section_id);

        let show_question = if is_expanded {
            None
        } else {
            Some(section_id)
        };
        
        html! {
            <>
                <div 
                    // class=classes!("section-item mb-5", is_selected.then(|| "selected mb-4"))
                    class={ "section-item mb-3" } 
                    key=section.id.clone().to_string()
                >
                    <div class="d-flex flex-wrap align-items-center justify-content-between">
                        <h6 class="text-purple-on noir-light is-size-18 lh-22">
                            <li>{ &section.title }</li>
                        </h6>
                        <div class="d-flex flex-wrap">
                            <button class="drop-hover-filter text-gray-purple-two"
                                onclick={ self.link.callback(move |_| CreatorMsg::EditSection(Some(section_clone.clone()))) }
                                disabled={ show_upset_section || show_upset_question }
                                style="border: none;">
                                <i class="fas fa-edit me-2"></i>
                                <span>{ "Editar" }</span>
                            </button>
                            <button class="drop-hover-filter-del text-red-delete ms-4"
                                onclick={ self.link.callback(move |_| CreatorMsg::DeleteSection(section_id.clone())) }
                                disabled={ show_upset_section || show_upset_question }
                                style="border: none;">
                                <i class="fas fa-trash me-2"></i>
                                <span>{ "Eliminar" }</span>
                            </button>
                        </div>
                    </div>
                    <p class="text-gray-strong noir-light is-size-16 mb-3">
                        { &section.description }
                    </p>
                    <div class="input-group align-items-center">
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 me-2" id="basic-addon1">{format!("{} preguntas", section.questions.len())}</span>
                        {
                            if self.section_validation(section) && self.props.is_evaluation {
                                html! {
                                    <span class="text-red-delete me-5" title="Falta asignar opciones correctas">
                                        // <i class="fas fa-info fas fa-lg"></i>
                                        <span class="badge rounded-pill bg-danger">{"incompleto"}</span>
                                    </span>
                                }
                            } else { html! {} }
                        }
                        <button class="drop-hover-filter text-primary-blue-dark" 
                            onclick={ self.link.callback(move |_| CreatorMsg::QuizCreatorActions(QuizCreatorMode::NewQuestion(upset_question.clone()))) }
                            disabled={ is_selected }
                            style="border: none;">
                            <i class="fas fa-plus me-2"></i>
                            <span>{ "Agregar" }</span>
                        </button>
                        <button class="drop-hover-filter text-purple-on ms-4" 
                            onclick={ self.link.callback(move |_| CreatorMsg::ShowQuestions(show_question)) }
                            disabled={ show_upset_section || show_upset_question }
                            style="border: none;">
                            <i class="fas fa-list me-2"></i>
                            <span>{ if is_expanded { "Ocultar" } else { "Mostrar" } }</span>
                        </button>
                    </div>
                </div>
                {
                    if is_expanded {
                        {
                            if !section.questions.is_empty() {
                                html! {
                                    <div class="card mb-5 py-3 px-4">
                                        { for section.questions.iter().map(|question| self.questions_item(section, question)) }
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="alert bg-light-sea-green noir-medium text-white mb-5" role="alert">
                                        <p class="mb-0">{"Agrega una pregunta a esta sección"}</p>
                                    </div>
                                }
                            }
                        }
                    } else {
                        html! {}
                    }
                }
            </>
        }
    }

    pub fn render_upset_section_item(&self) -> Html {
        let section_clone = self.upset_section.clone();
        html!{
            <div class="card section-creator mb-4">
                <div class="card-body">
                    <h3 class="text-primary-blue-dark text-camelcase noir-bold is-size-32 lh-43 mb-3">{"Nueva Sección del Cuestionario"}</h3>
                    <div class="form-group">
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Nombre*"}</label>
                            <input type="text" class="form-control input-style-universal" id="exampleFormControlInput1" placeholder="Título de la sección"
                                value={ self.upset_section.title.clone() }
                                oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateSectionTitle(e.value)) } />
                        </div>
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Descripción"}</label>
                            <input type="text" class="form-control input-style-universal" id="exampleFormControlInput1" placeholder="Descripción de la sección"
                                value={ self.upset_section.description.clone() }
                                oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateSectionDescription(e.value)) } />
                        </div>
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Observación"}</label>
                            <input type="text" class="form-control input-style-universal" id="exampleFormControlInput1" placeholder="Observación de la sección"
                                value={ self.upset_section.observation.clone() }
                                oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateSectionObservation(e.value)) } />
                        </div>

                        <div class="d-flex justify-content-center">
                            <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4" 
                                disabled={ self.upset_section.title.is_empty() } onclick={ self.link.callback(move |_| CreatorMsg::AddSection(Some(section_clone.clone()))) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Agregar Sección" }</span>
                            </button>
                            {            
                                if  !self.props.quiz.sections.is_empty() {
                                    html! {
                                        <button class="btn button-saved-eraser bg-gray-strong d-flex align-items-center justify-content-center mx-4" 
                                            onclick={ self.link.callback(move |_| CreatorMsg::AddSection(None)) }>
                                            <span class="text-white noir-bold is-size-16 lh-20">{ "Cancelar" }</span>
                                        </button>
                                    }
                                } else { html! {} }                              
                            }
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    pub fn render_upset_question_item(&self, section: &QuestionSection) -> Html {
        let section_id = section.id;
        // let question_clone = self.upset_question.clone();
        let prev_type = self.upset_question.question_type.clone();
        let options = self.upset_question.options.clone();

        html! {
            <div class="card question-editor mb-4">
                <div class="card-body">
                    <h5 class="text-primary-blue-light text-camelcase noir-medium is-size-24 lh-43 mb-3">{format!("Pregunta para: {}", section.title)}</h5>
                    <div class="form-group">
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Nueva Pregunta*"}</label>
                            <div class="input-group">
                                <textarea class="input input-style-universal px-3 py-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-12" aria-label="With textarea" 
                                    // style="min-height: 120px;"
                                    placeholder="Texto de la pregunta"
                                    rows="3"
                                    value={ self.upset_question.text.clone() }
                                    oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateQuestionText(e.value)) }>
                                </textarea>
                            </div>
                        </div>

                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Tipo de Pregunta"}</label>
                            <select class="form-select option-select-type" id="inputGroupSelect01" 
                                onchange={ self.link.callback(move |e: ChangeData| {
                                    let prev_type_clone = prev_type.clone();
                                    if let ChangeData::Select(select) = e {
                                        let value = select.value();
                                        let q_type = match value.as_str() {
                                            "multiple" => QuestionType::MultipleChoice,
                                            "truefalse" => QuestionType::TrueFalse,
                                            "text" => QuestionType::TextInput,
                                            _ => QuestionType::SingleChoice,
                                        };
                                        CreatorMsg::UpdateQuestionType(section_id, q_type, prev_type_clone)
                                    } else {
                                        CreatorMsg::UpdateQuestionType(section_id, QuestionType::SingleChoice, prev_type_clone)
                                    }
                                }) }>
                                <option value="single" selected={ self.upset_question.question_type == QuestionType::SingleChoice }>{"Selección única"}</option>
                                <option value="multiple" selected={ self.upset_question.question_type == QuestionType::MultipleChoice }>{"Selección múltiple"}</option>
                                <option value="truefalse" selected={ self.upset_question.question_type == QuestionType::TrueFalse }>{"Verdadero/Falso"}</option>
                                <option value="text" selected={ self.upset_question.question_type == QuestionType::TextInput }>{"Respuesta textual"}</option>
                            </select>
                        </div>

                        { self.render_options(&section, &self.upset_question, &options) }

                        <div class="d-flex justify-content-center">
                            <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4"
                                disabled={
                                    self.upset_question.text.is_empty()
                                    || (self.upset_question.question_type != QuestionType::TextInput && self.upset_question.options.is_empty())
                                    || (self.props.is_evaluation && !options.iter().any(|opt| opt.is_correct) && self.upset_question.question_type != QuestionType::TextInput)
                                }
                                onclick={ self.link.callback(move |_| CreatorMsg::AddQuestion(Some(section_id))) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Agregar Pregunta" }</span>
                            </button>
                            <button class="btn button-saved-eraser bg-gray-strong d-flex align-items-center justify-content-center mx-4" 
                                onclick={ self.link.callback(|_| CreatorMsg::AddQuestion(None)) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Cancelar" }</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    pub fn render_options(&self, section: &QuestionSection, question: &Question, options: &Vec<AnswerOption>) -> Html {
        let section_id = section.id;
        let question_id = question.id;
        let question_points = question.points;

        let is_o_expanded = self.is_o_expanded.is_some();

        let help = if !options.iter().any(|o| o.is_correct) && self.props.quiz.is_evaluation {
            html! {
                <div class="alert bg-light-sea-green noir-medium text-white" role="alert">
                    <p class="mb-0">{"Selecciona una opción como correcta"}</p>
                </div>
            }
        } else { html! {} };
        
        match self.upset_question.question_type {
            QuestionType::TextInput => html! { 
                <div>
                    { self.render_question_points( section_id, options.iter().find(|opt| opt.is_correct), question_points, self.upset_question.question_type.clone() )  }
                    <div class="alert bg-light-sea-green noir-medium text-white mt-2" role="alert">
                        <p class="mb-0">{"El usuario ingresará una respuesta textual"}</p>
                    </div>
                </div>
            },
            QuestionType::TrueFalse => html! {
                <div class="options-editor mb-4">
                    { self.render_question_points( section_id, options.iter().find(|opt| opt.is_correct), question_points, self.upset_question.question_type.clone() )  }
                    <div class="row mt-2 mb-4">
                        {
                            self.upset_question.options.iter().map(|option| {
                                let id = option.id;
                                let name = if option.text == "Verdadero" { "truefalse" } else { "truefalse" };
                                html!{
                                    <div class="col">
                                        <div class="form-check">
                                            <input class="form-check-input" value="" id="flexCheckDefault" 
                                                type="radio" 
                                                name={name} 
                                                checked={ option.is_correct }
                                                onclick={ self.link.callback(move |_| CreatorMsg::ToggleOptionCorrect(id)) }
                                                />
                                            <label class="form-check-label text-gray-strong noir-medium is-size-16" for="flexCheckDefault">
                                                { option.text.clone() }
                                            </label>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    { help }
                </div>
            },
            _ => html! {
                <div class="options-editor">
                    <div class="mb-3">
                        <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Opciones de respuesta *"}</label>
                        {
                            if !is_o_expanded {
                                html! {
                                    <div class="input-group mb-3">
                                        <input type="text" class="form-control input-style-universal" aria-label="Nueva opción" aria-describedby="button-addon2"
                                            placeholder="Nueva opción"
                                            value={ self.upset_option_text.clone() }
                                            oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateOptionText(e.value)) }
                                            style="border-top-right-radius: 0px !important; border-bottom-right-radius: 0px !important;" />
                                        <button class="btn bg-primary-blue-light noir-bold text-white" type="button" id="button-addon2" 
                                            onclick={ self.link.callback(move |_| CreatorMsg::AddOption(section_id, question_id)) }
                                            disabled={ self.upset_option_text.is_empty() }
                                            style="border-top-right-radius: 10px !important; border-bottom-right-radius: 10px !important;">
                                            {"Agregar Opción"}
                                        </button>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    { 
                        match self.upset_question.question_type {
                            QuestionType::MultipleChoice 
                            | QuestionType::TextInput => {
                                self.render_question_points( section_id, options.iter().find(|opt| opt.is_correct), question_points, self.upset_question.question_type.clone() ) 
                            }
                            _ => html!{}
                        }
                        
                    }
                    <div class="options-list">
                        { for options.iter().map(|opt| self.render_option_item(&section, opt, question_points, self.upset_question.question_type.clone() )) }
                    </div>
                    { if !options.is_empty() {
                            help
                        } else {
                            html! {}
                        }
                    }
                </div>
            },
        }
    }

    pub fn render_question_points(&self, section_id: Uuid, option: Option<&AnswerOption>, points: u32, question_type: QuestionType ) -> Html {
        log::info!("SOME OPTION {:?}", option);

        let text_dynamic_choice = match question_type {
            QuestionType::MultipleChoice 
            | QuestionType::TextInput
            | QuestionType::TrueFalse => html! { <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Puntos"}</label> },
            _ => html!{}
        };

        let text_single_choice = match question_type {
            QuestionType::SingleChoice => html! { <span class="input-group-text text-secondary-purple">{"Puntos"}</span> },
            _ => html!{}
        };

        let points_handle = html! {
            <div class="" style="width: 250px">
                { text_dynamic_choice }
                <div class="input-group input-group-sm">
                    { text_single_choice }
                    <button class="btn bg-primary-blue-light noir-bold text-white" type="button" id="button-addon1"
                        onclick={ self.link.callback(move |_| CreatorMsg::UpdateQuestionPoints(section_id, false))} >{"-1"}</button>
                    <input type="number" class="form-control input-style-universal text-center" 
                        id="exampleFormControlInput1" placeholder="0" aria-label="0" aria-describedby="inputGroup-sizing-sm"
                        min="1" value={format!("{}", points)}
                        style="border-top-right-radius: 0px !important; border-bottom-right-radius: 0px !important; width: 4.5rem; height: 3rem;" />
                    <button class="btn bg-primary-blue-light noir-bold text-white" type="button" id="button-addon2"
                        onclick={ self.link.callback(move |_| CreatorMsg::UpdateQuestionPoints(section_id, true))} >{"+1"}</button>
                </div>
            </div>
        };

        let is_correct = option.and_then(|opt| Some(opt.is_correct)).unwrap_or(false);

        if ( is_correct || question_type == QuestionType::TextInput ) && self.props.is_evaluation {
            points_handle
        } else {
            html! {}
        }
    }

    pub fn render_option_item(&self, section: &QuestionSection, option: &AnswerOption, points: u32, question_type: QuestionType) -> Html {
        let section_id = section.id;
        let option_id = option.id;
        let option_clone = option.clone();
        let is_editing = self.is_o_expanded == Some(option.clone());

        html! {
            <>
                <div class="option-item" key={ option.id.clone().to_string() }>
                    <div class="input-group d-flex justify-content-between mb-3">
                        <span class="input-group-text">
                            <div class="form-check">
                                <input class="form-check-input"
                                    name="flexRadioDefault" id="flexRadioDefault1"
                                    type={ match self.upset_question.question_type {
                                        QuestionType::MultipleChoice => "checkbox",
                                        _ => "radio",
                                    }}
                                    checked={ option.is_correct }
                                    onclick={ self.link.callback(move |_| CreatorMsg::ToggleOptionCorrect(option_id)) } />
                                <label class="form-check-label text-gray-strong noir-medium is-size-16" for="flexRadioDefault1">
                                    { &option.text }
                                </label>
                            </div>
                            {
                                if option.is_correct {
                                    html! {
                                        <label class="form-check-label">
                                            <span class="badge bg-success ms-2">{"Correcta"}</span>
                                        </label>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </span>
                        { 
                            if question_type == QuestionType::SingleChoice {
                                self.render_question_points( section_id, Some(option), points, question_type )
                            } else { html! {} }
                        }
                        <div class="d-flex flex-wrap align-content-center">
                            <button class="drop-hover-filter text-gray-purple-two"
                                onclick={ self.link.callback(move |_| CreatorMsg::EditOption(Some(option_clone.clone()))) }
                                disabled={ is_editing }
                                style="border: none; height: 3rem;">
                                <i class="fas fa-edit me-2"></i>
                                <span>{ "Editar" }</span>
                            </button>
                            <button class="drop-hover-filter-del text-red-delete ms-4"
                                onclick={ self.link.callback(move |_| CreatorMsg::DeleteOption(section_id, option_id)) }
                                style="border: none; height: 3rem;">
                                <i class="fas fa-trash me-2"></i>
                                <span>{ "Eliminar" }</span>
                            </button>
                        </div>
                    </div>
                    {
                        if is_editing {
                            html! {
                                { self.edit_option(&section) }
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </>
        }
    }
    pub fn questions_item(&self, section: &QuestionSection, question: &Question) -> Html {
        let section_id = section.id;
        let question_id = question.id;

        let section_clone = section.clone();
        let question_clone = question.clone();

        let show_upset_section = self.quiz_edit_mode == QuizCreatorMode::NewSection;
        let show_upset_question = self.quiz_edit_mode == QuizCreatorMode::NewQuestion(section_clone.clone());

        let question_editing = section.questions.iter().find(|s| s.id == section.id) == Some(&question_clone);

        html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between">
                <div>
                    <label class="text-primary-blue-light noir-light is-size-16 lh-43">{"* "}{ &question.text}</label>
                    <span class="badge bg-light-sea-green text-white noir-medium ms-2">
                        { format!("{} puntos", question.points) }
                    </span>
                    {
                        if question.points == 0 && !question.options.iter().any(|opt| opt.is_correct) && self.props.is_evaluation {
                            html! {
                                <span class="text-red-delete ms-2" title="Falta asignar opciones correctas">
                                    // <i class="fas fa-info fas fa-lg"></i>
                                    <span class="badge rounded-pill bg-danger">{"! opción correcta"}</span>
                                </span>
                            }
                        } else { html! {} }
                    }
                </div>
                <div class="d-flex flex-wrap">
                    <button class="drop-hover-filter text-gray-purple-two"
                        onclick={ self.link.callback(move |_| CreatorMsg::EditQuestion( Some(section_clone.clone()), Some(question_clone.clone()) )) }
                        disabled={ question_editing && (show_upset_section || show_upset_question) }
                        style="border: none;">
                        <i class="fas fa-edit me-2"></i>
                        <span>{ "Editar" }</span>
                    </button>
                    <button class="drop-hover-filter-del text-red-delete ms-4"
                        onclick={ self.link.callback(move |_| CreatorMsg::DeleteQuestion(section_id, question_id)) }
                        disabled={ question_editing && (show_upset_section || show_upset_question) }
                        style="border: none;">
                        <i class="fas fa-trash me-2"></i>
                        <span>{ "Eliminar" }</span>
                    </button>
                </div>
            </div>
        }
    }
}