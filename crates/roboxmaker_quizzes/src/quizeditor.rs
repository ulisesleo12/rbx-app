use yew::prelude::*;
use crate::{quiz::{QuestionSection, QuestionType}, quizcreator::{CreatorMsg, QuizCreator}};

impl QuizCreator {
    pub fn edit_section_item(&self) -> Html {
        html! {
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
                                disabled={ self.upset_section.title.is_empty() } onclick={ self.link.callback(move |_| CreatorMsg::UpdateSection) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Actualizar Sección" }</span>
                            </button>
                            <button class="btn button-saved-eraser bg-gray-strong d-flex align-items-center justify-content-center mx-4" 
                                onclick={ self.link.callback(|_| CreatorMsg::EditSection(None)) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Cancelar" }</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    pub fn edit_question_item(&self, section: &QuestionSection) -> Html {
        let section_id = section.id;
        let question_clone = self.upset_question.clone();
        let options = self.upset_question.options.clone();
        let prev_type = self.upset_question.question_type.clone();

        html! {
            <div class="card question-editor mb-4">
                <div class="card-body">
                    <h5 class="text-primary-blue-light text-camelcase noir-medium is-size-24 lh-43 mb-3">{format!("Pregunta para: {}", section.title)}</h5>
                    <div class="form-group">
                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Nueva Pregunta*"}</label>
                            <div class="input-group">
                                <textarea class="input input-style-universal px-3 py-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-12" aria-label="With textarea" 
                                    placeholder="Texto de la pregunta"
                                    value={ self.upset_question.text.clone() }
                                    oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateQuestionText(e.value)) }></textarea>
                            </div>
                        </div>

                        <div class="mb-3">
                            <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Tipo de Pregunta"}</label>
                            <select class="form-select option-select-type" id="inputGroupSelect01" 
                                onchange=self.link.callback(move |e: ChangeData| {
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
                                })>
                                <option value="single" selected={self.upset_question.question_type == QuestionType::SingleChoice}>{"Selección única"}</option>
                                <option value="multiple" selected={self.upset_question.question_type == QuestionType::MultipleChoice}>{"Selección múltiple"}</option>
                                <option value="truefalse" selected={self.upset_question.question_type == QuestionType::TrueFalse}>{"Verdadero/Falso"}</option>
                                <option value="text" selected={self.upset_question.question_type == QuestionType::TextInput}>{"Respuesta textual"}</option>
                            </select>
                        </div>

                        { self.render_options(&section, &self.upset_question, &options) }

                        <div class="d-flex justify-content-center">
                            <button class="btn button-saved-eraser d-flex align-items-center justify-content-center mx-4"
                                disabled={ self.upset_question.text.is_empty()
                                    || (self.props.is_evaluation && !self.upset_question.options.iter().any(|opt| opt.is_correct) && self.upset_question.question_type != QuestionType::TextInput)
                                    || (self.upset_question.question_type != QuestionType::TextInput && self.upset_question.options.is_empty()) }
                                onclick={ self.link.callback(move |_| CreatorMsg::UpdateQuestion(section_id, question_clone.clone())) }
                                >
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Actualizar" }</span>
                            </button>
                            <button class="btn button-saved-eraser bg-gray-strong d-flex align-items-center justify-content-center mx-4" 
                                onclick={ self.link.callback(move |_| CreatorMsg::EditQuestion(None, None)) }>
                                <span class="text-white noir-bold is-size-16 lh-20">{ "Cancelar" }</span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }

    pub fn edit_option(&self, section: &QuestionSection) -> Html {
        let section_id = section.id;
        html! {
            <div class="options-editor">
                <div class="mb-3">
                    <label for="exampleFormControlInput1" class="form-label noir-medium text-secondary-purple">{"Actualizar Opción*"}</label>
                    <div class="input-group mb-3">
                        <input type="text" class="form-control input-style-universal" aria-label="Nueva opción" aria-describedby="button-addon2"
                            placeholder="Nueva opción"
                            value={ self.upset_option_text.clone() }
                            oninput={ self.link.callback(|e: InputData| CreatorMsg::UpdateOptionText(e.value)) }
                            style="border-top-right-radius: 0px !important; border-bottom-right-radius: 0px !important;" />
                        <button class="btn bg-gray-strong noir-bold text-white" type="button" id="button-addon2" 
                            onclick={ self.link.callback(move |_| CreatorMsg::EditOption(None)) } >
                            {"Cancelar"}
                        </button>
                        <button class="btn bg-primary-blue-light noir-bold text-white" type="button" id="button-addon2" 
                            onclick={ self.link.callback(move |_| CreatorMsg::UpdateOption(section_id)) }
                            disabled={ self.upset_option_text.is_empty() }
                            style="border-top-right-radius: 10px !important; border-bottom-right-radius: 10px !important;">
                            {"Actualizar"}
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}