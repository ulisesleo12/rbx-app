use log::info;
use uuid::Uuid;
use serde::Serialize;
use chrono::Datelike;
use chrono::{NaiveDateTime, Timelike};
use std::collections::{HashMap, HashSet};


use roboxmaker_models::quiz_model;


#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizAnswerOptions {
    pub option_id: Uuid,
    pub option: String,
    pub question_id: Uuid,
    pub is_correct: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizUserMultipleChoices {
    pub id: Uuid,
    pub option_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizQuestion {
    pub section_id: Uuid,
    pub question_id: Uuid,
    pub question: String,
    pub question_order: i64,
    pub answer_options: Vec<QuizAnswerOptions>,
    pub points: i64,
    pub is_opinion_based: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizAnswers {
    pub answered_at: NaiveDateTime,
    pub question_id: Uuid,
    pub quiz_response_id: Uuid,
    pub user_answer_id: Uuid,
    pub answer_type: String,
    pub text_answer: Option<String>,
    pub is_true: Option<bool>,
    pub score: i64, 
    pub question: QuizQuestion,
    pub single_choice_option_id: Option<Uuid>,
    pub user_multiple_choices: Vec<QuizUserMultipleChoices>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizResponses {
    pub started_at: NaiveDateTime,
    pub completed_at: NaiveDateTime,
    pub status: String,
    pub total_score: i64,
    pub user_answers: Vec<QuizAnswers>,
    pub user_id: Uuid,
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizUserResponse {
    pub user_id: Uuid,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizUserAnswer {
    // pub title: String,
    pub quiz_users: Vec<QuizUserResponse>,
    pub quiz_responses: Vec<QuizResponses>,
}


pub fn quiz_with_user_answers_by_group(data_json: quiz_model::quiz_with_user_answers_by_user::ResponseData) -> QuizUserAnswer {

    let quiz_users = data_json
        .quiz_responses
        .iter()
        .map(|q_user| {
            QuizUserResponse {
                user_id: q_user.user.clone().and_then(|item| item.user_profile).and_then(|user_p| Some(user_p.user_id)).unwrap_or_default(),
                full_name: q_user.user.clone().and_then(|item| item.user_profile).and_then(|user_p| Some(user_p.full_name)).unwrap_or_default(),
                email: q_user.user.clone().and_then(|item| item.user_profile).and_then(|user_p| user_p.email).unwrap_or_default(),
            }
        }).collect();

    let quiz_responses = data_json.quiz_responses.iter().map(|item| {

        // let mut user_answers: Vec<QuizAnswers> = item.user_answers.iter()
        let user_answers = item.user_answers.iter()
            .enumerate()
            .map(|(idx, item)| {

            let user_multiple_choices = item.user_multiple_choices.iter().map(|item| {
                QuizUserMultipleChoices { 
                    id: item.id, 
                    option_id: item.option_id 
                }
            }).collect();

            let answer_options = item.question.answer_options.iter().map(|item| {
                QuizAnswerOptions { 
                    option_id: item.id, 
                    option: item.option.clone(), 
                    question_id: item.question_id,
                    is_correct: item.is_correct.unwrap_or(false),
                }
            }).collect();

            let is_opinion_based = !item.question.answer_options.iter().any(|opt| opt.is_correct.unwrap_or(false));
            let question = QuizQuestion {
                section_id: item.question.section_id,
                question_id: item.question.question_id,
                question: item.question.question.clone(),
                answer_options,
                points: item.question.points.unwrap_or(0),
                is_opinion_based,
                question_order: (idx + 1) as i64,
            };
            QuizAnswers {
                answered_at: item.answered_at,
                quiz_response_id: item.quiz_response_id,
                user_answer_id: item.user_answer_id,
                question_id: item.question_id,
                answer_type: item.answer_type.clone().unwrap_or_default(),
                text_answer: item.text_answer.clone(),
                is_true: item.is_true,
                score: item.score,
                question,
                single_choice_option_id: item.single_choice_option_id,
                user_multiple_choices,
            }
        }).collect();

        // user_answers.sort_by_key(|a| a.question.question_order);

        QuizResponses {
            started_at: item.started_at,
            completed_at: item.completed_at,
            status: item.status.clone(),
            total_score: item.total_score,
            user_answers,
            user_id: item.user_id.unwrap_or_default(),
        }
    }).collect();

    QuizUserAnswer {
        quiz_users,
        quiz_responses,
    }
}


fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuizStats {
    pub total_students: usize,
    pub fastest_user: Option<(String, String)>,
    pub users_by_speed: Vec<(String, i64)>, // (name, duration_in_seconds)
    pub average_score: f64,
    pub top_scorers: Vec<(String, f64, String)>, // (name, score, time) 
}

pub fn generate_quiz_statistics(quiz_data: &QuizUserAnswer) -> QuizStats {
    let mut question_count: HashMap<Uuid, (String, usize)> = HashMap::new(); // question_id -> (question_text, count)
    let mut user_durations: Vec<(String, i64)> = Vec::new(); // full_name, duration in seconds
    let mut score_by_user: HashMap<String, f64> = HashMap::new(); // cambio a f64
    let mut all_scores = Vec::new();

    let mut user_score_data: Vec<(Uuid, String, f64, i64)> = Vec::new(); // cambio a f64

    for (i, response) in quiz_data.quiz_responses.iter().enumerate() {
        let duration = (response.completed_at - response.started_at).num_seconds();
        let full_name = quiz_data.quiz_users.get(i).map(|u| u.full_name.clone()).unwrap_or_default();

        // Calcular puntaje con la nueva lógica de división de puntos
        let total_score: f64 = response.user_answers.iter().map(|answer| {
            let mut question_score = 0.0; // cambio a f64

            // Pregunta de opción única
            if let Some(selected_id) = answer.single_choice_option_id {
                if answer
                    .question
                    .answer_options
                    .iter()
                    .any(|opt| opt.option_id == selected_id && opt.is_correct)
                {
                    question_score = answer.question.points as f64;
                }
            }
            // Pregunta de opción múltiple
            else if !answer.user_multiple_choices.is_empty() {
                let selected_ids: Vec<Uuid> = answer
                    .user_multiple_choices
                    .iter()
                    .filter_map(|opt| opt.option_id)
                    .collect();

                // Contar cuántas opciones correctas hay en total
                let total_correct_options = answer
                    .question
                    .answer_options
                    .iter()
                    .filter(|opt| opt.is_correct)
                    .count();

                if total_correct_options > 0 {
                    // Calcular puntos por opción correcta
                    let points_per_correct_option = answer.question.points as f64 / total_correct_options as f64;

                    // Contar cuántas opciones correctas seleccionó el usuario
                    let user_correct_count = selected_ids.iter().filter(|id| {
                        answer
                            .question
                            .answer_options
                            .iter()
                            .any(|opt| opt.option_id == **id && opt.is_correct)
                    }).count();

                    // Calcular puntaje: puntos por opciones correctas
                    let calculated_score = user_correct_count as f64 * points_per_correct_option;

                    // Truncar a 2 decimales sin redondear
                    let truncated_score = (calculated_score * 100.0).floor() / 100.0;

                    // Asegurar que el puntaje no sea negativo
                    question_score = truncated_score.max(0.0);
                }
            } else {
                if answer.answer_type == String::from("TEXT_INPUT") && answer.is_true.unwrap_or(false) {
                    question_score = answer.question.points as f64;
                }
            }

            question_score
        }).sum();

        user_durations.push((full_name.clone(), duration));
        score_by_user.insert(full_name.clone(), total_score);
        all_scores.push(total_score);
        user_score_data.push((response.user_id, full_name.clone(), total_score, duration));

        for answer in &response.user_answers {
            let entry = question_count.entry(answer.question_id)
                .or_insert((answer.question.question.clone(), 0));
            entry.1 += 1;
        }
    }

    // 1. Número total de estudiantes
    let total_students = quiz_data.quiz_users.len();

    // 2. Usuario más veloz en responder
    let fastest_user = user_durations
        .iter()
        .min_by_key(|(_, dur)| *dur)
        .map(|(name, dur)| {
            (name.clone(), format_duration(*dur))
        });

    // 3. Usuarios ordenados del más veloz al menos veloz
    user_durations.sort_by_key(|(_, dur)| *dur);

    // 4. Puntuación media
    let average_score = if !all_scores.is_empty() {
        let sum: f64 = all_scores.iter().sum();
        let truncated_avg = (sum / all_scores.len() as f64 * 100.0).floor() / 100.0;
        truncated_avg
    } else {
        0.0
    };

    // 5. Usuario(s) con la mayor puntuación
    // Ordenar por puntaje descendente
    user_score_data.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let top_scorers: Vec<(String, f64, String)> = user_score_data
        .into_iter()
        .map(|(_, name, score, duration)| {
            (name, score, format_duration(duration))
        })
        .collect();

    QuizStats {
        total_students,
        fastest_user,
        users_by_speed: user_durations,
        average_score,
        top_scorers,
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct QuestionStats {
    pub id: Uuid,
    pub question: String,
    pub options: Vec<String>,
    pub responses: Vec<(String, String)>, // (user, selected option)
    pub option_counts: Vec<(String, usize)>, // Option -> Quantity
    pub correct_count: Option<usize>,
    pub incorrect_count: Option<usize>,
    pub is_opinion_based: bool,
}

pub fn generate_question_statistics(quiz_data: &QuizUserAnswer) -> Vec<QuestionStats> {
    
    let user_map: HashMap<Uuid, String> = quiz_data
        .quiz_users
        .iter()
        .map(|u| (u.user_id, u.full_name.clone()))
        .collect();

    let mut question_stats_map: HashMap<Uuid, (QuestionStats, HashMap<String, HashSet<Uuid>>, usize, usize)> = HashMap::new();
    
    // Vector to maintain the original order of the questions
    let mut ordered_question_ids = Vec::new();

    for response in &quiz_data.quiz_responses {
        let user_id = response.user_id;

        let user_name = user_map
            .get(&user_id)
            .cloned()
            .unwrap_or_else(|| "Desconocido".to_string());

        for answer in &response.user_answers {
            let question_id = answer.question_id;
            let question_text = answer.question.question.clone();
            let answer_options = &answer.question.answer_options;

            let is_opinion_based = if answer.answer_type == "TEXT_INPUT" {
                answer.is_true.is_none()
            } else {
                !answer_options.iter().any(|opt| opt.is_correct)
            };
            
            // We only add the id to the ordered vector the first time we see the question
            if !question_stats_map.contains_key(&question_id) {
                ordered_question_ids.push(question_id);
            }

            // Initialize if not exists
            let entry = question_stats_map.entry(question_id).or_insert_with(|| {
                let options = answer
                    .question
                    .answer_options
                    .iter()
                    .map(|opt| opt.option.clone())
                    .collect::<Vec<_>>();
                (
                    QuestionStats {
                        id: question_id,
                        question: question_text.clone(),
                        options,
                        responses: Vec::new(),
                        option_counts: Vec::new(),
                        correct_count: None,
                        incorrect_count: None,
                        is_opinion_based,
                    },
                    HashMap::new(), // opción => set de user_ids
                    0,
                    0,
                )
            });

            // Reuse mutable reference
            let (stats, option_user_map, correct_count, incorrect_count) = entry;

            // Extract the option answered
            if let Some(single_id) = answer.single_choice_option_id {
                if let Some(opt) = answer_options.iter().find(|o| o.option_id == single_id) {
                    let option_text = opt.option.clone();
                    let user_set = option_user_map.entry(option_text.clone()).or_insert_with(HashSet::new);
                    if user_set.insert(user_id) {
                        stats.responses.push((user_name.clone(), option_text));
                    }

                    if !is_opinion_based {
                        if opt.is_correct {
                            *correct_count += 1;
                        } else {
                            *incorrect_count += 1;
                        }
                    }
                }
            }

            // Multiple selected options
            if !answer.user_multiple_choices.is_empty() {
                for multiple in &answer.user_multiple_choices {
                    if let Some(opt_id) = multiple.option_id {
                        if let Some(opt) = answer_options.iter().find(|o| o.option_id == opt_id) {
                            let option_text = opt.option.clone();
                            let user_set = option_user_map.entry(option_text.clone()).or_insert_with(HashSet::new);
                            if user_set.insert(user_id) {
                                stats.responses.push((user_name.clone(), option_text));
                            }

                            if !is_opinion_based {
                                if opt.is_correct {
                                    *correct_count += 1;
                                } else {
                                    *incorrect_count += 1;
                                }
                            }
                        }
                    }
                }
            }
            
            // Extract the option answered (TEXT_INPUT)
            if answer.answer_type == String::from("TEXT_INPUT") {
                if let Some(is_correct) = answer.is_true {
                    let user_answer = answer.text_answer.clone().unwrap_or_else(|| "[Respuesta vacía]".to_string());
                    
                    // Keep the user's textual response
                    stats.responses.push((user_name.clone(), user_answer));

                    if !is_opinion_based {
                        if is_correct {
                            *correct_count += 1;
                        } else {
                            *incorrect_count += 1;
                        }
                    }
                }
            }
        }
    }

    // We convert a neighbor keeping the original order
    ordered_question_ids
        .into_iter()
        .filter_map(|id| question_stats_map.remove(&id))
        .map(|(mut stats, option_map, correct, incorrect)| {
            let mut option_counts: Vec<(String, usize)> = option_map
                .into_iter()
                .map(|(opt, user_set)| (opt, user_set.len()))
                .collect();
            option_counts.sort_by(|a, b| b.1.cmp(&a.1));

            stats.option_counts = option_counts;

            if !stats.is_opinion_based {
                stats.correct_count = Some(correct);
                stats.incorrect_count = Some(incorrect);
            }

            stats
        })
        .collect()
}


pub fn score_distribution(quiz_data: &QuizUserAnswer) -> Vec<(i64, usize)> {
    let mut distribution: HashMap<i64, usize> = HashMap::new();

    for response in &quiz_data.quiz_responses {
        let mut total_score = 0;

        for answer in &response.user_answers {
            let mut question_score = 0;

            // Pregunta de opción única
            if let Some(selected_id) = answer.single_choice_option_id {
                if answer
                    .question
                    .answer_options
                    .iter()
                    .any(|opt| opt.option_id == selected_id && opt.is_correct)
                {
                    question_score = answer.question.points;
                }
            }
            // Pregunta de opción múltiple
            else if !answer.user_multiple_choices.is_empty() {
                let selected_ids: Vec<Uuid> = answer
                    .user_multiple_choices
                    .iter()
                    .filter_map(|opt| opt.option_id)
                    .collect();

                // Contar cuántas opciones correctas hay en total
                let total_correct_options = answer
                    .question
                    .answer_options
                    .iter()
                    .filter(|opt| opt.is_correct)
                    .count();

                if total_correct_options > 0 {
                    // Calcular puntos por opción correcta
                    let points_per_correct_option = answer.question.points as f64 / total_correct_options as f64;

                    // Contar cuántas opciones correctas seleccionó el usuario
                    let user_correct_count = selected_ids.iter().filter(|id| {
                        answer
                            .question
                            .answer_options
                            .iter()
                            .any(|opt| opt.option_id == **id && opt.is_correct)
                    }).count();

                    // Contar cuántas opciones incorrectas seleccionó el usuario
                    // let user_incorrect_count = selected_ids.iter().filter(|id| {
                    //     answer
                    //         .question
                    //         .answer_options
                    //         .iter()
                    //         .any(|opt| opt.option_id == **id && !opt.is_correct)
                    // }).count();

                    // Calcular puntaje: puntos por opciones correctas
                    let calculated_score = user_correct_count as f64 * points_per_correct_option;

                    // Asegurar que el puntaje no sea negativo
                    question_score = calculated_score.max(0.0).round() as i64;
                }
            } else {
                if answer.answer_type == String::from("TEXT_INPUT") && answer.is_true.unwrap_or(false) {
                    question_score = answer.question.points;
                }
            }

            // Sumar el puntaje de esta pregunta al total
            total_score += question_score;
        }

        *distribution.entry(total_score).or_insert(0) += 1;
    }

    let mut result: Vec<(i64, usize)> = distribution.into_iter().collect();
    result.sort_by_key(|(score, _)| *score); // Ordenar por puntuación
    info!("score_distribution {:?}", result);
    result
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

#[derive(Debug, Clone)]
pub struct UserQuizSummary {
    pub full_name: String,
    pub completed_at: String,
    pub score: f64,
    pub correct: usize,
    pub incorrect: usize,
    pub duration: String,
    pub details: Vec<UserAnswerDetail>,
}

#[derive(Debug, Clone)]
pub struct UserAnswerDetail {
    pub question_number: usize,
    pub answer_type: String,
    pub question: String,
    pub answer: String,
    pub correct: bool,
}


pub fn build_quiz_summary(quiz_data: &QuizUserAnswer) -> Vec<UserQuizSummary> {
    let mut summaries = vec![];

    for quiz_response in &quiz_data.quiz_responses {
        let user = quiz_data
            .quiz_users
            .iter()
            .find(|u| u.user_id == quiz_response.user_id);

        if let Some(user_info) = user {
            let mut score = 0.0f64; // Cambio a f64
            let mut correct = 0;
            let mut incorrect = 0;
            let mut details = vec![];

            // Calcular puntaje con la nueva lógica de división de puntos
            let total_score: f64 = quiz_response.user_answers.iter().map(|ans| {
                let mut question_score = 0.0f64;

                // Pregunta de opción única
                if let Some(selected_id) = ans.single_choice_option_id {
                    if ans.question.answer_options.iter().any(|opt| opt.option_id == selected_id && opt.is_correct) {
                        question_score = ans.question.points as f64;
                    }
                }
                // Pregunta de opción múltiple
                else if !ans.user_multiple_choices.is_empty() {
                    let selected_ids: Vec<Uuid> = ans.user_multiple_choices
                        .iter()
                        .filter_map(|mc| mc.option_id)
                        .collect();

                    // Contar cuántas opciones correctas hay en total
                    let total_correct_options = ans.question
                        .answer_options
                        .iter()
                        .filter(|opt| opt.is_correct)
                        .count();

                    if total_correct_options > 0 {
                        // Calcular puntos por opción correcta
                        let points_per_correct_option = ans.question.points as f64 / total_correct_options as f64;

                        // Contar cuántas opciones correctas seleccionó el usuario
                        let user_correct_count = selected_ids.iter().filter(|id| {
                            ans.question
                                .answer_options
                                .iter()
                                .any(|opt| opt.option_id == **id && opt.is_correct)
                        }).count();

                        // Calcular puntaje: puntos por opciones correctas
                        let calculated_score = user_correct_count as f64 * points_per_correct_option;

                        // Truncar a 2 decimales sin redondear
                        let truncated_score = (calculated_score * 100.0).floor() / 100.0;

                        // Asegurar que el puntaje no sea negativo
                        question_score = truncated_score.max(0.0);
                    }
                } else {
                    // Pregunta TEXT_INPUT
                    if ans.answer_type == String::from("TEXT_INPUT") && ans.is_true.unwrap_or(false) {
                        question_score = ans.question.points as f64;
                    }
                }

                question_score
            }).sum();

            score = total_score;

            for (i, ans) in quiz_response.user_answers.iter().enumerate() {
                let mut is_answer_correct = false;
                let answer_type = ans.answer_type.clone();

                if let Some(selected_id) = ans.single_choice_option_id {
                    // Opción única
                    is_answer_correct = ans.question
                        .answer_options
                        .iter()
                        .any(|opt| opt.option_id == selected_id && opt.is_correct);
                } else if !ans.user_multiple_choices.is_empty() {
                    // Múltiples opciones
                    let selected_ids: Vec<Uuid> = ans.user_multiple_choices
                        .iter()
                        .filter_map(|mc| mc.option_id)
                        .collect();

                    let correct_ids: Vec<Uuid> = ans.question
                        .answer_options
                        .iter()
                        .filter(|opt| opt.is_correct)
                        .map(|opt| opt.option_id)
                        .collect();

                    let all_correct_selected = selected_ids.iter().all(|id| correct_ids.contains(id));
                    let selected_exactly_all_correct = selected_ids.len() == correct_ids.len();

                    is_answer_correct = all_correct_selected && selected_exactly_all_correct;
                } else {
                    // Pregunta TEXT_INPUT
                    if ans.answer_type == String::from("TEXT_INPUT") {
                        is_answer_correct = ans.is_true.unwrap_or(false);
                    }
                }

                if is_answer_correct {
                    correct += 1;
                } else {
                    incorrect += 1;
                }

                // Obtener respuesta seleccionada (single o múltiples opciones)
                let selected_answer = if let Some(id) = ans.single_choice_option_id {
                    // Respuesta de opción única
                    ans.question
                        .answer_options
                        .iter()
                        .find(|opt| opt.option_id == id)
                        .map(|opt| opt.option.clone())
                        .unwrap_or("Respuesta desconocida".to_string())
                } else if !ans.user_multiple_choices.is_empty() {
                    // Respuestas múltiples
                    let selected_options: Vec<String> = ans.user_multiple_choices
                        .iter()
                        .filter_map(|mc| mc.option_id)
                        .filter_map(|oid| {
                            ans.question
                                .answer_options
                                .iter()
                                .find(|opt| opt.option_id == oid)
                                .map(|opt| opt.option.clone())
                        })
                        .collect();

                    if selected_options.is_empty() {
                        "Sin respuesta válida".to_string()
                    } else {
                        selected_options.join(", ")
                    }
                } else {
                    if ans.answer_type == String::from("TEXT_INPUT") {
                        ans.text_answer.clone().unwrap_or_default()
                    } else {
                        "Sin respuesta".to_string()
                    }
                };

                details.push(UserAnswerDetail {
                    question_number: i + 1,
                    answer_type,
                    question: ans.question.question.clone(),
                    answer: selected_answer,
                    correct: is_answer_correct,
                });
            }

            // Formato de hora: "03:10 - 12 May 2025"
            let time = quiz_response.completed_at.time();
            let date: chrono::NaiveDate = quiz_response.completed_at.date();
            let completed_at_formatted = format!("{:02}:{:02} - {} {} {}", time.hour12().1, time.minute(), date.day0(), month_name(date.month()), date.year());

            // Duración: HH:MM:SS
            let duration = (quiz_response.completed_at - quiz_response.started_at).num_seconds();
            let duration_formatted = format_duration(duration);

            summaries.push(UserQuizSummary {
                full_name: user_info.full_name.clone(),
                completed_at: completed_at_formatted,
                score, // Ahora es f64 con decimales truncados
                correct,
                incorrect,
                duration: duration_formatted,
                details,
            });
        }
    }
    summaries
}


// Auxiliary function to convert duration "00:05:31" to seconds
pub fn parse_duration_seconds(duration_str: &str) -> u64 {
    let parts: Vec<&str> = duration_str.split(':').collect();
    if parts.len() != 3 {
        return 0;
    }
    let h = parts[0].parse::<u64>().unwrap_or(0);
    let m = parts[1].parse::<u64>().unwrap_or(0);
    let s = parts[2].parse::<u64>().unwrap_or(0);
    h * 3600 + m * 60 + s
}


#[derive(Debug, Clone)]
pub struct QuestionSummary {
    pub question_id: Uuid,  // Asumo que tus preguntas tienen un ID
    pub answer_type: String,
    pub question_number: usize, 
    pub question_points: i64,
    pub question_text: String,
    pub total_correct: usize,
    pub total_incorrect: usize,
    pub user_responses: Vec<UserQuestionResponse>,
}

#[derive(Debug, Clone)]
pub struct UserQuestionResponse {
    pub user_name: String,
    pub user_answer: String,
    pub is_correct: bool,
    pub completed_at: String,
}

pub fn build_question_summary(quiz_data: &QuizUserAnswer) -> Vec<QuestionSummary> {
    // Primero obtenemos todas las preguntas en orden (asumiendo que todas las respuestas tienen las mismas preguntas)
    let questions_in_order: Vec<_> = quiz_data.quiz_responses
        .first() // Tomamos el primer quiz como referencia para el orden
        .map(|response| response.user_answers.iter().enumerate().map(|(i, ans)| (ans.question.question_id, i + 1, ans.question.question.clone())))
        .into_iter()
        .flatten()
        .collect();

    // Creamos un mapa para acumular los datos pero manteniendo el orden
    let mut question_map: HashMap<Uuid, QuestionSummaryBuilder> = HashMap::new();
    let mut ordered_question_ids = Vec::new();

    // Procesamos las respuestas como antes
    for quiz_response in &quiz_data.quiz_responses {
        let user = quiz_data
            .quiz_users
            .iter()
            .find(|u| u.user_id == quiz_response.user_id);

        if let Some(user_info) = user {
            for (i, ans) in quiz_response.user_answers.iter().enumerate() {
                let question_id = ans.question.question_id;
                let answer_type = ans.answer_type.clone();
                
                // Solo agregamos el ID al vector ordenado la primera vez que vemos la pregunta
                if !question_map.contains_key(&question_id) {
                    ordered_question_ids.push(question_id);
                }

                // Resto del procesamiento igual que antes...
                let mut is_answer_correct = false;
                
                if let Some(selected_id) = ans.single_choice_option_id {
                    is_answer_correct = ans.question
                        .answer_options
                        .iter()
                        .any(|opt| opt.option_id == selected_id && opt.is_correct);
                } else if !ans.user_multiple_choices.is_empty() {
                    let selected_ids: Vec<Uuid> = ans.user_multiple_choices
                        .iter()
                        .filter_map(|mc| mc.option_id)
                        .collect();

                    let correct_ids: Vec<Uuid> = ans.question
                        .answer_options
                        .iter()
                        .filter(|opt| opt.is_correct)
                        .map(|opt| opt.option_id)
                        .collect();

                    let all_correct_selected = selected_ids.iter().all(|id| correct_ids.contains(id));
                    let selected_exactly_all_correct = selected_ids.len() == correct_ids.len();

                    is_answer_correct = all_correct_selected && selected_exactly_all_correct;
                } else {
                    if ans.is_true.is_some() {
                        is_answer_correct = ans.is_true.unwrap_or(false);
                    }
                }

                let selected_answer = if let Some(id) = ans.single_choice_option_id {
                    ans.question
                        .answer_options
                        .iter()
                        .find(|opt| opt.option_id == id)
                        .map(|opt| opt.option.clone())
                        .unwrap_or("Respuesta desconocida".to_string())
                } else if !ans.user_multiple_choices.is_empty() {
                    let selected_options: Vec<String> = ans.user_multiple_choices
                        .iter()
                        .filter_map(|mc| mc.option_id)
                        .filter_map(|oid| {
                            ans.question
                                .answer_options
                                .iter()
                                .find(|opt| opt.option_id == oid)
                                .map(|opt| opt.option.clone())
                        })
                        .collect();

                    if selected_options.is_empty() {
                        "Sin respuesta válida".to_string()
                    } else {
                        selected_options.join(", ")
                    }
                } else {
                    if ans.answer_type == String::from("TEXT_INPUT") {
                        ans.text_answer.clone().unwrap_or_default()
                    } else {
                        "Sin respuesta".to_string()
                    }
                };

                let time = quiz_response.completed_at.time();
                let date: chrono::NaiveDate = quiz_response.completed_at.date();
                let completed_at_formatted = format!("{:02}:{:02} - {} {}", 
                    time.hour12().1, time.minute(), month_name(date.month()), date.year());

                let entry = question_map.entry(question_id).or_insert_with(|| {
                    // Buscamos el número de pregunta en el orden original
                    let binding = (question_id, i + 1, ans.question.question.clone());

                    let (_, question_number, question_text) = questions_in_order.iter()
                        .find(|(id, _, _)| *id == question_id)
                        .unwrap_or(&binding);
                    
                    QuestionSummaryBuilder {
                        question_id,
                        answer_type,
                        question_number: *question_number,
                        question_points: ans.question.points,
                        question_text: question_text.clone(),
                        total_correct: 0,
                        total_incorrect: 0,
                        user_responses: Vec::new(),
                    }
                });

                if is_answer_correct {
                    entry.total_correct += 1;
                } else {
                    entry.total_incorrect += 1;
                }

                entry.user_responses.push(UserQuestionResponse {
                    user_name: user_info.full_name.clone(),
                    user_answer: selected_answer,
                    is_correct: is_answer_correct,
                    completed_at: completed_at_formatted,
                });
            }
        }
    }

    // Convertimos el HashMap a Vec<QuestionSummary> manteniendo el orden original
    ordered_question_ids.into_iter()
        .filter_map(|id| question_map.remove(&id))
        .map(|builder| QuestionSummary {
            question_id: builder.question_id,
            answer_type: builder.answer_type,
            question_number: builder.question_number,
            question_points: builder.question_points,
            question_text: builder.question_text,
            total_correct: builder.total_correct,
            total_incorrect: builder.total_incorrect,
            user_responses: builder.user_responses,
        })
        .collect()
}

// Estructura auxiliar para construir el resumen
pub struct QuestionSummaryBuilder {
    question_id: Uuid,
    answer_type: String,
    question_number: usize, 
    question_points: i64, 
    question_text: String,
    total_correct: usize,
    total_incorrect: usize,
    user_responses: Vec<UserQuestionResponse>,
}

#[derive(Debug, Clone)]
pub struct TextQuestionSummary {
    pub question_id: Uuid,
    pub question_number: usize,
    pub question_text: String,
    pub total_correct: usize,
    pub total_incorrect: usize,
    pub user_responses: Vec<UserTextResponse>,
}

#[derive(Debug, Clone)]
pub struct UserTextResponse {
    pub quiz_response_id: Uuid,
    pub user_answer_id: Uuid,
    pub user_name: String,
    pub text_answer: String,
    pub is_correct: Option<bool>,
    pub completed_at: String,
}

pub fn build_text_questions_summary(quiz_data: &QuizUserAnswer) -> Vec<TextQuestionSummary> {
    // Primero obtenemos todas las preguntas de tipo TEXT_INPUT en orden
    let text_questions_in_order: Vec<_> = quiz_data.quiz_responses
        .first() // Tomamos el primer quiz como referencia para el orden
        .map(|response| {
            response.user_answers
                .iter()
                .enumerate()
                .filter(|(_, ans)| ans.answer_type == "TEXT_INPUT")
                .map(|(i, ans)| (ans.question.question_id, i + 1, ans.question.question.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    // Creamos un mapa para acumular los datos pero manteniendo el orden
    let mut question_map: HashMap<Uuid, TextQuestionSummaryBuilder> = HashMap::new();
    let mut ordered_question_ids = Vec::new();

    // Procesamos las respuestas
    for quiz_response in &quiz_data.quiz_responses {
        let user = quiz_data
            .quiz_users
            .iter()
            .find(|u| u.user_id == quiz_response.user_id);

        if let Some(user_info) = user {
            for ans in quiz_response.user_answers.iter() {
                // Solo procesamos preguntas de tipo TEXT_INPUT
                if ans.answer_type != "TEXT_INPUT" {
                    continue;
                }

                let question_id = ans.question.question_id;
                
                // Solo agregamos el ID al vector ordenado la primera vez que vemos la pregunta
                if !question_map.contains_key(&question_id) {
                    ordered_question_ids.push(question_id);
                }

                // Obtenemos la respuesta de texto
                let text_answer = ans.text_answer
                    .as_ref()
                    .unwrap_or(&"Sin respuesta".to_string())
                    .clone();

                // Formateamos la fecha y hora
                let time = quiz_response.completed_at.time();
                let date: chrono::NaiveDate = quiz_response.completed_at.date();
                let completed_at_formatted = format!("{:02}:{:02} - {} {}", 
                    time.hour12().1, time.minute(), month_name(date.month()), date.year());

                let entry = question_map.entry(question_id).or_insert_with(|| {
                    // Buscamos el número de pregunta en el orden original
                    let binding = (question_id, 1, ans.question.question.clone());

                    let (_, question_number, question_text) = text_questions_in_order.iter()
                        .find(|(id, _, _)| *id == question_id)
                        .unwrap_or(&binding);
                    
                    TextQuestionSummaryBuilder {
                        question_id,
                        question_number: *question_number,
                        question_text: question_text.clone(),
                        total_correct: 0,
                        total_incorrect: 0,
                        user_responses: Vec::new(),
                    }
                });

                // Actualizamos los contadores basados en ans.is_true
                match ans.is_true {
                    Some(true) => entry.total_correct += 1,
                    Some(false) => entry.total_incorrect += 1,
                    None => {} // No contamos como correcta ni incorrecta
                }

                entry.user_responses.push(UserTextResponse {
                    quiz_response_id: ans.quiz_response_id,
                    user_answer_id: ans.user_answer_id,
                    user_name: user_info.full_name.clone(),
                    text_answer,
                    is_correct: ans.is_true,
                    completed_at: completed_at_formatted,
                });
            }
        }
    }

    // Convertimos el HashMap a Vec<TextQuestionSummary> manteniendo el orden original
    ordered_question_ids.into_iter()
        .filter_map(|id| question_map.remove(&id))
        .map(|builder| TextQuestionSummary {
            question_id: builder.question_id,
            question_number: builder.question_number,
            question_text: builder.question_text,
            total_correct: builder.total_correct,
            total_incorrect: builder.total_incorrect,
            user_responses: builder.user_responses,
        })
        .collect()
}

// Estructura auxiliar para construir el resumen
struct TextQuestionSummaryBuilder {
    question_id: Uuid,
    question_number: usize,
    question_text: String,
    total_correct: usize,
    total_incorrect: usize,
    user_responses: Vec<UserTextResponse>,
}