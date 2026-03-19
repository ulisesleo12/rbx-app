use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use code_location::code_location;

use crate::quiz_charts::QuizCharts;
use crate::quiz_handle::build_text_questions_summary;
use crate::{quiz_handle::{build_question_summary, build_quiz_summary, generate_quiz_statistics, QuizUserAnswer}};
use roboxmaker_models::quiz_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

#[derive(Debug, Clone, PartialEq)]
pub enum ResultsPerStudentSortBy {
    CompletedAt,
    Name,
    ScoreDuration,
}


#[derive(Debug, Clone, PartialEq)]
pub enum SortResultRespBy {
    // Number,
    Name,
    Correct,
    Incorrect
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortResultBy {
    Number,
    Correct,
    Incorrect
}


pub struct QuizMetrics {
    link: ComponentLink<Self>,
    props: Props,
    graphql_task: Option<GraphQLTask>,
    mark_answer_task: Option<RequestTask>,
    expanded_index: Option<usize>,
    results_per_student_sort_by: ResultsPerStudentSortBy,
    expanded_details_question: Option<Uuid>,
    show_all: bool,
    sort_q_result_by: SortResultBy,
    sort_q_result_resp_by: SortResultRespBy,
    expanded_resolve_question: Option<Uuid>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz_user_answer: Option<QuizUserAnswer>,
}

#[derive(Debug)]
pub enum Message {
    ToggleSummaryDetail(usize),
    ChangeSort(ResultsPerStudentSortBy),
    ToggleSummaryByQuestion(Uuid),
    ToggleResolveQuestion(Uuid),
    ToggleShowAll,
    SortQuestionResultBy(SortResultBy),
    SortQuestionResultRespBy(SortResultRespBy),
    MarkUserTextAnswer(Uuid, Uuid, bool),
    MarkUserTextAnswerResponse(Option<quiz_model::update_user_answer::ResponseData>),
}

impl Component for QuizMetrics {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        QuizMetrics { 
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            mark_answer_task: None,
            expanded_index: None,
            results_per_student_sort_by: ResultsPerStudentSortBy::CompletedAt,
            expanded_details_question: None,
            show_all: false,
            sort_q_result_by: SortResultBy::Number,
            // sort_q_result_resp_by: SortResultRespBy::Number,
            sort_q_result_resp_by: SortResultRespBy::Name,
            expanded_resolve_question: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("QuizMetrics: {:?}", msg);
        match msg {
            Message::ToggleSummaryDetail(index) => {
                if self.expanded_index == Some(index) {
                    // If it is already expanding, we close it
                    self.expanded_index = None;
                } else {
                    // We expand this index, closing any other
                    self.expanded_index = Some(index);
                }
            }
            Message::ChangeSort(sort_by) => {
                self.results_per_student_sort_by = sort_by
            }
            Message::ToggleSummaryByQuestion(id) => {
                if self.expanded_details_question == Some(id) {
                    self.expanded_details_question = None;
                } else {
                    self.expanded_details_question = Some(id);
                }
            }
            Message::ToggleResolveQuestion(id) => {
                if self.expanded_resolve_question == Some(id) {
                    self.expanded_resolve_question = None;
                } else {
                    self.expanded_resolve_question = Some(id);
                }
            }
            Message::ToggleShowAll => {
                self.show_all = !self.show_all;
            }
            Message::SortQuestionResultBy(sort_by) => {
                self.sort_q_result_by = sort_by;
            }
            Message::SortQuestionResultRespBy(sort_by) => {
                self.sort_q_result_resp_by = sort_by;
            }
            Message::MarkUserTextAnswer(response_id, answer_id, value ) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = quiz_model::update_user_answer::Variables {
                        quiz_response_id: response_id,
                        user_answer_id: answer_id,
                        is_true: value
                    };
    
                    let task = quiz_model::UpdateUserAnswer::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::MarkUserTextAnswerResponse(response)
                        },
                    );
                    self.mark_answer_task = Some(task);
                }
            }
            Message::MarkUserTextAnswerResponse(response) => {
                info!("{:?}", response);
            }
        }
        true
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


        if let Some(quiz_user_answer) = &self.props.quiz_user_answer {
            let quiz_stats = generate_quiz_statistics(quiz_user_answer);
            let mut question_summary = build_question_summary(quiz_user_answer);
            let mut question_text_summary = build_text_questions_summary(quiz_user_answer);

            let total: f64 = quiz_stats.top_scorers.iter().map(|(_, score, _)| *score).fold(0.0, f64::max);
            let n_users = quiz_stats.top_scorers.iter().filter(|item| item.1 == total).map(|item| item.clone()).collect::<Vec<_>>();

            let fastest_user = quiz_stats.fastest_user.clone().unwrap_or_default().0;
            let fastest_user_parts: Vec<&str> = fastest_user.split_whitespace().collect();
            let fastest_user_name = fastest_user_parts.first().unwrap_or(&"");

            let top_scorers = if self.show_all {
                quiz_stats.top_scorers.clone()
            } else {
                quiz_stats.top_scorers.iter().take(3).cloned().collect()
            };

            let leaderboard_table = top_scorers.iter().enumerate().map(|(idx, item)| {
                let user = item.0.clone();
                let user_parts: Vec<&str> = user.split_whitespace().collect();
                let user_name = user_parts.first().unwrap_or(&"");
                html! {
                    <tr>
                        <th class="noir-regular text-dark text-center" scope="row">{ idx + 1 }{".ª"}</th>
                        <td class="txt-capitalize noir-regular text-dark">{ user_name }</td>
                        <td class="noir-regular text-dark text-center">{ item.1 }</td>
                        <td class="noir-regular text-dark text-center">{ item.2.clone() }</td>
                    </tr>
                }
            }).collect::<Html>();

            let build_quiz_summary = build_quiz_summary(quiz_user_answer);

            let mut sorted_summary = build_quiz_summary.clone();
            
            sorted_summary.sort_by(|a, b| {
                match self.results_per_student_sort_by {
                    ResultsPerStudentSortBy::CompletedAt => a.completed_at.cmp(&b.completed_at),
                    ResultsPerStudentSortBy::Name => a.full_name.cmp(&b.full_name),
                    ResultsPerStudentSortBy::ScoreDuration => b.correct.cmp(&a.correct),
                    // ResultsPerStudentSortBy::ScoreDuration => {
                    //     let a_score = a.correct as u64 + parse_duration_seconds(&a.duration);
                    //     let b_score = b.correct as u64 + parse_duration_seconds(&b.duration);
                    //     a_score.cmp(&b_score)
                    // },
                }
            });

            let quiz_summary_table = sorted_summary.iter().enumerate().map(|(idx, item)| {
                let user = item.full_name.clone();
                let user_parts: Vec<&str> = user.split_whitespace().collect();
                let user_name = user_parts.first().unwrap_or(&"");

                let onclick =  self.link.callback(move |_| Message::ToggleSummaryDetail(idx));

                let sub_summary_table = item.details.iter()
                    // .filter(|item| item.answer_type != String::from("TEXT_INPUT"))
                    .enumerate()
                    .map(|(idx, item)| {
                    
                    let icon_correct = if item.correct {
                        html! {
                            <span class="text-cyan-turquesa ms-3" key="icon-is-correct">
                                <i class="fas fa-check fas fa-lg"></i>
                            </span>
                        }
                    } else {
                        html! {
                            <span class="text-red-delete ms-3" key="icon-is-incorrect">
                                <i class="fas fa-times fas fa-lg"></i>
                            </span>
                        }
                    };

                    html! {
                        <tr>
                            <th class="noir-regular fw-normal text-dark text-center" scope="row">{ idx + 1 }</th>
                            <td class="txt-capitalize noir-regular text-dark text-center">{ item.question.clone() }</td>
                            <td class="noir-regular text-dark text-center">{ item.answer.clone() }</td>
                            <td class="noir-regular text-dark text-center">{ icon_correct }</td>
                        </tr>
                    }
                }).collect::<Html>();

                let is_visible = self.expanded_index == Some(idx);

                let icon_arrow = if is_visible {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-down">
                            <i class="fas fa-caret-down fas fa-lg"></i>
                        </span>
                    }
                } else {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-right">
                            <i class="fas fa-caret-right fas fa-lg"></i>
                        </span>
                    }
                };

                html! {
                    <>
                        <tr onclick={ onclick } style="cursor: pointer;">
                            // <th class="noir-regular text-gray-purple" scope="row">{ idx + 1 }</th>
                            <th class="noir-regular text-dark fw-normal text-center" scope="row">{ icon_arrow }</th>
                            <td class="txt-capitalize noir-regular text-dark fw-normal text-center">{ user_name }</td>
                            <td class="noir-regular text-dark fw-normal text-center">{ item.completed_at.clone() }</td>
                            <td class="noir-regular text-dark fw-normal text-center">{ item.score }</td>
                            <td class="noir-regular text-dark fw-normal text-center">{ item.correct }</td>
                            <td class="noir-regular text-dark fw-normal text-center">{ item.incorrect }</td>
                            <td class="noir-regular text-dark fw-normal text-center">{ item.duration.clone() }</td>
                        </tr>
                        {
                            if is_visible {
                                html! {
                                    <tr>
                                        <td colspan="7" class="table-active p-4">
                                            <div class="card" style="width: 100%;">
                                                <div class="card-body">
                                                    <table class="table table-striped mt-0 mb-0">
                                                        <thead>
                                                            <tr class="table-info">
                                                                <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Pregunta"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Respuesta"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Marca"}</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            { sub_summary_table }
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </>
                }
            }).collect::<Html>();

            question_summary
                .sort_by(|a, b| 
                    match self.sort_q_result_by {
                        SortResultBy::Correct => b.total_correct.cmp(&a.total_correct),
                        SortResultBy::Incorrect => b.total_incorrect.cmp(&a.total_incorrect),
                        _ => std::cmp::Ordering::Equal,
                    }
                );
            
            question_text_summary
                .sort_by(|a, b| 
                    match self.sort_q_result_resp_by {
                        SortResultRespBy::Name => b.question_text.cmp(&a.question_text),
                        SortResultRespBy::Correct => b.total_correct.cmp(&a.total_correct),
                        SortResultRespBy::Incorrect => b.total_incorrect.cmp(&a.total_incorrect),
                        // _ => std::cmp::Ordering::Equal,
                    }
                );

            let quiz_summary_table_questions =    
                question_summary
                .iter()
                // .filter(|item| item.answer_type != String::from("TEXT_INPUT"))
                .enumerate()
                .map(|(idx, item)| {
                let id = item.question_id;
                let onclick =  self.link.callback(move |_| Message::ToggleSummaryByQuestion(id));

                let user_summary_table_questions = item.user_responses.iter().enumerate().map(|(_idx, response)| {
                    let icon_correct = if response.is_correct {
                        html! {
                            <span class="ms-3 text-cyan-turquesa" key="icon-is-correct">
                                <i class="fas fa-check fas fa-lg"></i>
                            </span>
                        }
                    } else {
                        html! {
                            <span class="ms-3 text-red-delete" key="icon-is-incorrect">
                                <i class="fas fa-times fas fa-lg"></i>
                            </span>
                        }
                    };

                html! {
                        <tr>
                            //    <th class="noir-regular text-dark" scope="row">{ idx + 1 }</th>
                            <td class="txt-capitalize noir-regular text-dark text-center">{ &response.user_name }</td>
                            <td class="noir-regular text-dark text-center">{ &response.user_answer }</td>
                            <td class="noir-regular text-dark text-center">{ icon_correct }</td>
                        </tr>
                    }
                }).collect::<Html>();

                let is_visible = self.expanded_details_question == Some(id);

                let icon_arrow = if is_visible {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-down">
                            <i class="fas fa-caret-down fas fa-lg"></i>
                        </span>
                    }
                } else {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-right">
                            <i class="fas fa-caret-right fas fa-lg"></i>
                        </span>
                    }
                };

                html! {
                    <>
                        <tr onclick={ onclick } style="cursor: pointer;">
                            <th class="noir-regular text-dark fw-normal text-center" scope="row">{idx + 1}{ icon_arrow }</th>
                            <td class="txt-capitalize noir-regular text-dark text-center">{ &item.question_text }</td>
                            <td class="noir-regular text-dark text-center">{ &item.question_points }</td>
                            <td class="noir-regular text-dark text-center">{ item.total_correct }</td>
                            <td class="noir-regular text-dark text-center">{ item.total_incorrect }</td>
                        </tr>
                        {
                            if is_visible {
                                html! {
                                    <tr>
                                        <td colspan="7" class="table-active p-4">
                                            <div class="card" style="width: 100%;">
                                                <div class="card-body">
                                                    <table class="table table-striped mt-0 mb-0">
                                                        <thead>
                                                            <tr class="table-info">
                                                                // <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Alumno"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Respuesta"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Marca"}</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            { user_summary_table_questions }
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            } else { html! {} }
                        }
                    </>
                }
            }).collect::<Html>();

            let quiz_summary_resp_table_questions =    
                question_text_summary
                .iter()
                .enumerate()
                .map(|(idx, item)| {
                let id = item.question_id;
                let onclick =  self.link.callback(move |_| Message::ToggleResolveQuestion(id));

                let user_summary_table_questions = item.user_responses.iter().enumerate().map(|(_idx, response)| {
                    let response_id = response.quiz_response_id;
                    let answer_id = response.user_answer_id;
                    let is_correct = response.is_correct.unwrap_or(false) && response.is_correct.is_some();
                    let is_incorrect = !response.is_correct.unwrap_or(true) && response.is_correct.is_some();

                    let on_correct =  self.link.callback(move |_| Message::MarkUserTextAnswer(response_id, answer_id, true));
                    let on_incorrect =  self.link.callback(move |_| Message::MarkUserTextAnswer(response_id, answer_id, false));

                    html! {
                        <tr>
                            <td class="txt-capitalize noir-regular text-dark text-center">{ &response.user_name }</td>
                            <td class="noir-regular text-dark text-center">{ &response.text_answer }</td>
                            <td class="noir-regular text-dark d-flex justify-content-center">
                                // <div class="form-check form-switch d-flex flex-nowrap">
                                    // <input class="form-check-input results-checked-input me-4" type="checkbox" id="flexSwitchCheckChecked" checked={ true } />
                                    // <label class="form-check-label" for="flexSwitchCheckChecked">{ "Correcta" }</label>
                                // </div>
                                <div class="d-flex flex-nowrap">
                                    <div class="form-check me-4" onclick={on_correct}>
                                        <input class="form-check-input correct-checked-input" type="checkbox" value="" id="flexCheckDefault" checked={is_correct} />
                                        <label class="form-check-label" for="flexCheckDefault">
                                        {"Correcta"}
                                        </label>
                                    </div>
                                    <div class="form-check" onclick={on_incorrect}>
                                        <input class="form-check-input incorrect-checked-input" type="checkbox" value="" id="flexCheckChecked" checked={is_incorrect} />
                                        <label class="form-check-label" for="flexCheckChecked">
                                            {"Incorrecta"}
                                        </label>
                                    </div>
                                </div>
                            </td>
                        </tr>
                    }
                }).collect::<Html>();

                let is_visible = self.expanded_resolve_question == Some(id);

                let icon_arrow = if is_visible {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-down">
                            <i class="fas fa-caret-down fas fa-lg"></i>
                        </span>
                    }
                } else {
                    html! {
                        <span class="ms-3 text-dark" key="arrow-right">
                            <i class="fas fa-caret-right fas fa-lg"></i>
                        </span>
                    }
                };

                html! {
                    <>
                        <tr onclick={ onclick } style="cursor: pointer;">
                            <th class="noir-regular text-dark fw-normal text-center" scope="row">{idx + 1}{ icon_arrow }</th>
                            <td class="txt-capitalize noir-regular text-dark text-center">{ &item.question_text }</td>
                            <td class="noir-regular text-dark text-center">{ item.total_correct }</td>
                            <td class="noir-regular text-dark text-center">{ item.total_incorrect }</td>
                        </tr>
                        {
                            if is_visible {
                                html! {
                                    <tr>
                                        <td colspan="7" class="table-active p-4">
                                            <div class="card" style="width: 100%;">
                                                <div class="card-body">
                                                    <table class="table table-striped mt-0 mb-0">
                                                        <thead>
                                                            <tr class="table-info">
                                                                // <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Alumno"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Respuesta"}</th>
                                                                <th class="noir-bold text-blue-two text-center" scope="col">{"Resolver"}</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            { user_summary_table_questions }
                                                        </tbody>
                                                    </table>
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                }
                            } else { html! {} }
                        }
                    </>
                }
            }).collect::<Html>();

            let text_input_question_exists = !question_text_summary.is_empty();

            html! {
                <>
                    <div class="d-flex justify-content-between pt-5">
                        <div class="d-flex flex-column align-items-center">
                            <span class="text-primary-blue-dark text-uppercase noir-regular is-size-18 lh-18">
                                { "N.º de alumnos" }
                            </span>
                            <span class="text-purple-on noir-bold is-size-36 lh-36">
                                // { quiz_user_answer.quiz_users.len() }
                                { quiz_stats.total_students }
                            </span>
                        </div>
                        <div class="d-flex flex-column align-items-center">
                            <span class="text-primary-blue-dark text-uppercase noir-regular is-size-18 lh-18">
                                { "Puntuación media" }
                            </span>
                            <span class="text-purple-on noir-bold is-size-36 lh-36">
                                { quiz_stats.average_score.round() }
                            </span>
                        </div>
                        <div class="d-flex flex-column align-items-center">
                            <span class="text-primary-blue-dark text-uppercase noir-regular is-size-18 lh-18">
                                { "Máxima puntuación" }
                            </span>
                            <span class="text-purple-on noir-bold is-size-36 lh-36">
                                { total }
                            </span>
                            <span class="text-gray-purple-two noir-regular is-size-16 lh-16">
                                { n_users.len() }{ " alumnos" }
                            </span>
                        </div>
                        <div class="d-flex flex-column align-items-center">
                            <span class="text-primary-blue-dark text-uppercase noir-regular is-size-18 lh-18">
                                { "Más veloz" }
                            </span>
                            <span class="text-purple-on noir-bold is-size-36 lh-36">
                                { quiz_stats.fastest_user.clone().unwrap_or_default().1 }
                            </span>
                            <span class="text-gray-purple-two txt-capitalize noir-regular is-size-16 lh-16">
                                { fastest_user_name }
                            </span>
                        </div>
                    </div>

                    <QuizCharts quiz_user_answer={ Some(quiz_user_answer.clone()) } /> // APEX CHARTS

                    {
                        if text_input_question_exists {
                            html! {
                                <div class="pb-6 pt-5">
                                    <div class="card" style="width: 100%;">
                                        <div class="card-body">
                                            <div class="d-flex justify-content-between">
                                                <h5 class="card-title text-primary-blue-dark noir-regular is-size-18 lh-18">{ "Preguntas para resolver" }</h5>
                                                <div class="d-flex">
                                                    <h6 class="card-title text-black-50 noir-regular is-size-14 lh-18 my-1">{ "ORDENAR POR" }</h6>
                                                    // <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultRespBy(SortResultRespBy::Number))} >
                                                    //     <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault1" id="flexRadioDefault1" 
                                                    //         checked={self.sort_q_result_resp_by == SortResultRespBy::Number} />
                                                    //     <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault1">
                                                    //         { "Número" }
                                                    //     </label>
                                                    // </div>
                                                    <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultRespBy(SortResultRespBy::Name))} >
                                                        <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault2" id="flexRadioDefault2" 
                                                            checked={self.sort_q_result_resp_by == SortResultRespBy::Name} />
                                                        <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault2">
                                                            { "Nombre" }
                                                        </label>
                                                    </div>
                                                    <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultRespBy(SortResultRespBy::Correct))} >
                                                        <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault3" id="flexRadioDefault3" 
                                                            checked={self.sort_q_result_resp_by == SortResultRespBy::Correct} />
                                                        <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault3">
                                                            { "Correcto" }
                                                        </label>
                                                    </div>
                                                    <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultRespBy(SortResultRespBy::Incorrect))} >
                                                        <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault4" id="flexRadioDefault4" 
                                                            checked={self.sort_q_result_resp_by == SortResultRespBy::Incorrect} />
                                                        <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault4">
                                                            { "Incorrecto" }
                                                        </label>
                                                    </div>
                                                </div>
                                            </div>
                                            <table class="table table-striped mt-4 mb-0">
                                                <thead>
                                                    <tr class="table-info">
                                                        <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                                        <th class="noir-bold text-blue-two text-center" scope="col">{"Pregunta"}</th>
                                                        <th class="noir-bold text-blue-two text-center" scope="col">{"Correcto"}</th>
                                                        <th class="noir-bold text-blue-two text-center" scope="col">{"Incorrecto"}</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    { quiz_summary_resp_table_questions }
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>
                                </div>

                            }
                        } else {
                            html! {}
                        }
                    }

                    <div class="pb-6 pt-5">
                        <div class="card" style="width: 100%;">
                            <div class="card-body">
                                <h5 class="card-title text-primary-blue-dark noir-regular is-size-18 lh-18">{ "Tabla de clasificación" }</h5>
                                <table class="table table-striped mt-4 mb-0">
                                    <thead>
                                        <tr class="table-info">
                                            <th class="noir-bold text-blue-purple text-center" scope="col">{"Puesto"}</th>
                                            <th class="noir-bold text-blue-purple" scope="col">{"Nombre"}</th>
                                            <th class="noir-bold text-blue-purple text-center" scope="col">{"Puntuación"}</th>
                                            <th class="noir-bold text-blue-purple text-center" scope="col">{"Tiempo"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { leaderboard_table }
                                    </tbody>
                                </table>
                                <div class="d-flex justify-content-center">
                                    {
                                        if quiz_stats.top_scorers.len() > 2 {
                                            html! {
                                                <div class="text-center mt-3">
                                                    <button class="btn btn-light bg-white noir-regular is-size-12 text-gray-strong" style="border: none !important;"
                                                        onclick={self.link.callback(|_| Message::ToggleShowAll)}>
                                                        { if self.show_all { "Mostrar menos" } else { "Mostrar más" } }

                                                        {
                                                            if self.show_all {
                                                                html! {
                                                                    <span class="ms-3" key="arrow-up">
                                                                        <i class="fas fa-caret-up fas fa-lg"></i>
                                                                    </span>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <span class="ms-3" key="arrow-down">
                                                                        <i class="fas fa-caret-down fas fa-lg"></i>
                                                                    </span>
                                                                }
                                                            }
                                                        }
                                                    </button>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="pb-6 pt-5">
                        <div class="card" style="width: 100%;">
                            <div class="card-body">
                                <div class="d-flex justify-content-between">
                                    <h5 class="card-title text-primary-blue-dark noir-regular is-size-18 lh-18">{ "Resultados por alumno" }</h5>
                                    <div class="d-flex flex-wrap">
                                        <span class="noir-regular text-gray-strong text-uppercase is-size-14 me-4">{ "ORDENAR POR" }</span>
                                        <div class="form-check" onclick={self.link.callback(|_| Message::ChangeSort(ResultsPerStudentSortBy::CompletedAt))}>
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" checked={ self.results_per_student_sort_by == ResultsPerStudentSortBy::CompletedAt } name="flexRadioDefaultEnv" id="flexRadioDefault1" />
                                            <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault1">
                                                { "Envío" }
                                            </label>
                                        </div>
                                        <div class="form-check mx-4" onclick={self.link.callback(|_| Message::ChangeSort(ResultsPerStudentSortBy::Name))}>
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" checked={ self.results_per_student_sort_by == ResultsPerStudentSortBy::Name } name="flexRadioDefaultNam" id="flexRadioDefault2" />
                                            <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault2">
                                                { "Nombre" }
                                            </label>
                                        </div>
                                        <div class="form-check" onclick={self.link.callback(|_| Message::ChangeSort(ResultsPerStudentSortBy::ScoreDuration))}>
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" checked={ self.results_per_student_sort_by == ResultsPerStudentSortBy::ScoreDuration } name="flexRadioDefaultCorrect" id="flexRadioDefault3" />
                                            // <label class="form-check-label noir-medium  text-gray-dark" for="flexRadioDefault3">
                                            //     { "Correcto + Hora" }
                                            // </label>
                                            <label class="form-check-label noir-medium text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault3">
                                                { "Correcto" }
                                            </label>
                                        </div>
                                    </div>
                                </div>
                                <table class="table table-striped mt-4 mb-0">
                                    <thead>
                                        <tr class="table-info">
                                            <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Alumno"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Enviado"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Puntuación"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Correcto"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Incorrecto"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Tiempo"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { quiz_summary_table }
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>

                    <div class="pb-6 pt-5">
                        <div class="card" style="width: 100%;">
                            <div class="card-body">
                                <div class="d-flex justify-content-between">
                                    <h5 class="card-title text-primary-blue-dark noir-regular is-size-18 lh-18">{ "Resultados por pregunta" }</h5>
                                    <div class="d-flex">
                                        <h6 class="card-title text-black-50 noir-regular is-size-14 lh-18 my-1">{ "ORDENAR POR" }</h6>
                                        <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultBy(SortResultBy::Number))} >
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault" id="flexRadioDefault1" 
                                                checked={self.sort_q_result_by == SortResultBy::Number} />
                                            <label class="form-check-label noir-medium  text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault1">
                                                { "Número" }
                                            </label>
                                        </div>
                                        <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultBy(SortResultBy::Correct))} >
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault" id="flexRadioDefault2" 
                                                checked={self.sort_q_result_by == SortResultBy::Correct} />
                                            <label class="form-check-label noir-medium  text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault2">
                                                { "Correcto" }
                                            </label>
                                        </div>
                                        <div class="form-check mx-3" onclick={self.link.callback( |_| Message::SortQuestionResultBy(SortResultBy::Incorrect))} >
                                            <input class="form-check-input cursor-pointer results-checked-input" type="radio" name="flexRadioDefault" id="flexRadioDefault3" 
                                                checked={self.sort_q_result_by == SortResultBy::Incorrect} />
                                            <label class="form-check-label noir-medium  text-gray-dark is-size-14 cursor-pointer" for="flexRadioDefault3">
                                                { "Incorrecto" }
                                            </label>
                                        </div>
                                    </div>
                                </div>
                                <table class="table table-striped mt-4 mb-0">
                                    <thead>
                                        <tr class="table-info">
                                            <th class="noir-bold text-blue-two" scope="col">{""}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Pregunta"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Puntuación asignada"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Correcto"}</th>
                                            <th class="noir-bold text-blue-two text-center" scope="col">{"Incorrecto"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { quiz_summary_table_questions }
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                </>
            }
        } else {
            html! {}
        }
    }
}