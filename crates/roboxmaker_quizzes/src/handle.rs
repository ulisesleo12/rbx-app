use crate::quiz::{AnswerOption, Question, QuestionSection, QuestionType, Quiz, QuizResponses, UserProfile};

use roboxmaker_models::quiz_model;
use uuid::Uuid;


pub fn get_quiz(quiz_json: quiz_model::quiz_by_id::QuizByIdQuizzesGroupByPk, user_id: Uuid) -> Quiz {
    let sections = quiz_json.quiz.question_sections.iter().map(|section| {
        let questions = section.questions.iter().map(|question| {
            let options = question.answer_options.iter().map(|option| AnswerOption {
                id: option.id,
                text: option.option.clone(),
                is_correct: option.is_correct.unwrap_or(false),
            }).collect();
            
            Question {
                id: question.question_id,
                text: question.question.clone(),
                question_type: match question.question_type.as_ref().unwrap() {
                    quiz_model::quiz_by_id::RoboxQuestionTypeEnum::SingleChoice => QuestionType::SingleChoice,
                    quiz_model::quiz_by_id::RoboxQuestionTypeEnum::MultipleChoice => QuestionType::MultipleChoice,
                    quiz_model::quiz_by_id::RoboxQuestionTypeEnum::TrueFalse => QuestionType::TrueFalse,
                    quiz_model::quiz_by_id::RoboxQuestionTypeEnum::TextInput => QuestionType::TextInput,
                    _ => QuestionType::SingleChoice
                },
                options,
                points: question.points.unwrap_or(0) as u32,
            }
        }).collect();

        QuestionSection {
            id: section.section_id,
            title: section.title.clone(),
            description: section.description.clone().unwrap_or_default(),
            observation: section.observation.clone().unwrap_or_default(),
            questions,
        }
    }).collect();

    let quiz_responses = quiz_json.quiz.quiz_responses.iter().filter(|item| item.user_id == Some(user_id)).map(|quiz_resp| {
        QuizResponses {
            quiz_response_id: quiz_resp.quiz_response_id,
            quiz_id: quiz_resp.quiz_id,
            user_id: quiz_resp.user_id.unwrap_or_default(),
            status: quiz_resp.status.clone(),
            started_at: quiz_resp.started_at,
            completed_at: quiz_resp.completed_at,
        }
    }).collect();
    
    let author_profile = quiz_json.quiz.user_profile.clone().and_then(|item| {
        Some(UserProfile{
            user_id: item.user_id,
            full_name: item.full_name,
            pic_path: item.pic_path.unwrap_or("https://files.roboxmaker.com/uploads/avatar.png".to_string())
        })
    });

    Quiz {
        id: quiz_json.quiz.quiz_id,
        title: quiz_json.quiz.title.unwrap_or_default(),
        description: quiz_json.quiz.description.unwrap_or_default(),
        create_at: quiz_json.quiz.created_at,
        state: quiz_json.quiz.state.unwrap_or_default(),
        sections,
        quiz_responses,
        author_id: quiz_json.quiz.author_id,
        author_profile,
        is_evaluation: quiz_json.quiz.is_evaluation
    }
}