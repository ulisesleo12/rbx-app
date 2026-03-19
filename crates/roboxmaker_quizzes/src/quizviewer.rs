use yew::prelude::*;
use crate::quiz::{Quiz, QuestionSection, Question, AnswerOption, QuestionType};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct QuizViewerProps {
    pub quiz: Quiz,
}

pub struct QuizViewer {
    props: QuizViewerProps,
    _link: ComponentLink<Self>,
}

impl Component for QuizViewer {
    type Message = ();
    type Properties = QuizViewerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        QuizViewer {
            _link: link,
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }    

    fn view(&self) -> Html {
        let quiz = &self.props.quiz;
        
        html! {
            <div class="quiz-view mx-4 my-4">
                <div class="quiz-header mb-4">
                    <h3 class="text-primary-blue-dark text-camelcase noir-bold is-size-48 text-center lh-43 mb-3">{&quiz.title}</h3>
                    {
                        if !quiz.description.is_empty() {
                            html! { 
                                <p class="text-gray-strong noir-light is-size-16 text-justify mb-3">{&quiz.description}</p> 
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>

                <div class="quiz-sections">
                    {for quiz.sections.iter().map(|section| self.render_section(section))}
                </div>
            </div>
        }
    }
}

impl QuizViewer {
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
        html! {
            <div class="question mb-4" key={ question.id.clone().to_string() }>
                <div class="question-header">
                    <label class="text-primary-blue-light noir-light is-size-16 lh-43 mb-3">{ &question.text }</label>
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
                </div>

                { 
                    match question.question_type {
                        QuestionType::TextInput => html! {
                            <div class="answer-input mt-2">
                                <textarea class="form-control" rows="3" disabled=true></textarea>
                            </div>
                        },
                        _ => html! {
                            <div class="options mt-2">
                                { for question.options.iter().map(|option| self.render_option(question, option)) }
                            </div>
                        }
                    } 
                }
            </div>
        }
    }

    fn render_option(&self, question: &Question,  option: &AnswerOption) -> Html {
        let input_type = match question.question_type {
            QuestionType::SingleChoice => "radio",
            QuestionType::MultipleChoice => "checkbox",
            _ => "radio", // Por defecto, aunque no debería ocurrir
        };
        html! {
            <div class="form-check mb-2">
                <input 
                    class="form-check-input" 
                    type={ input_type }
                    checked={ option.is_correct }
                    disabled={ true }
                />
                <label class="form-check-label text-gray-strong noir-medium is-size-16">
                    { &option.text }
                    {
                        if option.is_correct {
                            html! { <span class="badge bg-success ms-2">{"Correcta"}</span> }
                        } else {
                            html! {}
                        }
                    }
                </label>
            </div>
        }
    }
}