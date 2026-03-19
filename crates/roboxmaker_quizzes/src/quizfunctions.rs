use uuid::Uuid;
use std::collections::HashSet;


use crate::quiz::{AnswerOption, Question, QuestionSection, QuestionType, Quiz, UserAnswer};

impl Quiz {
    // Add a section
    pub fn add_section(&mut self, section: QuestionSection) {
        self.sections.push(section);
    }

    // Delete a section for your ID
    pub fn remove_section(&mut self, section_id: Uuid) {
        self.sections.retain(|s| s.id != section_id);
    }

    // Get a section for your ID
    pub fn get_section(&self, section_id: Uuid) -> Option<&QuestionSection> {
        self.sections.iter().find(|s| s.id == section_id)
    }

    //Obtain a mutable section for your ID
    pub fn get_section_mut(&mut self, section_id: Uuid) -> Option<&mut QuestionSection> {
        self.sections.iter_mut().find(|s| s.id == section_id)
    }

    // Add question to a section
    pub fn add_question(&mut self, section_id: Uuid, question: Question) -> bool {
        if let Some(section) = self.get_section_mut(section_id) {
            section.questions.push(question);
            true
        } else {
            false
        }
    }

    // Delete a question from a section
    pub fn remove_question(&mut self, section_id: Uuid, question_id: Uuid) -> bool {
        if let Some(section) = self.get_section_mut(section_id) {
            section.questions.retain(|q| q.id != question_id);
            true
        } else {
            false
        }
    }

    // Get question about your ID
    pub fn get_question(&self, section_id: Uuid, question_id: Uuid) -> Option<&Question> {
        self.get_section(section_id)
            .and_then(|section| section.questions.iter().find(|q| q.id == question_id))
    }

    // Get mutable question about your ID
    pub fn get_question_mut(&mut self, section_id: Uuid, question_id: Uuid) -> Option<&mut Question> {
        self.get_section_mut(section_id)
            .and_then(|section| section.questions.iter_mut().find(|q| q.id == question_id))
    }

    // Add option to a question
    pub fn add_option(&mut self, section_id: Uuid, question_id: Uuid, option: AnswerOption) -> bool {
        if let Some(question) = self.get_question_mut(section_id, question_id) {
            question.options.push(option);
            true
        } else {
            false
        }
    }

    // Delete option by ID
    pub fn remove_option(&mut self, section_id: Uuid, question_id: Uuid, option_id: Uuid) -> bool {
        if let Some(question) = self.get_question_mut(section_id, question_id) {
            question.options.retain(|opt| opt.id != option_id);
            true
        } else {
            false
        }
    }

    // Get option by ID
    pub fn get_option(&self, section_id: Uuid, question_id: Uuid, option_id: Uuid) -> Option<&AnswerOption> {
        self.get_question(section_id, question_id)
            .and_then(|q| q.options.iter().find(|opt| opt.id == option_id))
    }

    // Get mutable option by ID
    pub fn get_option_mut(&mut self, section_id: Uuid, question_id: Uuid, option_id: Uuid) -> Option<&mut AnswerOption> {
        self.get_question_mut(section_id, question_id)
            .and_then(|q| q.options.iter_mut().find(|opt| opt.id == option_id))
    }

    // Update Quiz
    pub fn update_quiz(&mut self, updated: Quiz) {
        self.id = updated.id;
        self.title = updated.title;
        self.description = updated.description;
        self.sections = updated.sections;
    }

    // Update a section for your ID
    pub fn update_section(&mut self, updated_section: QuestionSection) -> bool {
        if let Some(section) = self.sections.iter_mut().find(|s| s.id == updated_section.id) {
            *section = updated_section;
            true
        } else {
            false
        }
    }

    // Update a question within a section
    pub fn update_question(&mut self, section_id: Uuid, updated_question: Question) -> bool {
        if let Some(section) = self.get_section_mut(section_id) {
            if let Some(question) = section.questions.iter_mut().find(|q| q.id == updated_question.id) {
                *question = updated_question;
                return true;
            }
        }
        false
    }

    // Update an option within a question
    pub fn update_option(&mut self, section_id: Uuid, question_id: Uuid, updated_option: AnswerOption) -> bool {
        if let Some(question) = self.get_question_mut(section_id, question_id) {
            if let Some(option) = question.options.iter_mut().find(|opt| opt.id == updated_option.id) {
                *option = updated_option;
                return true;
            }
        }
        false
    }

    // Delete all options from question
    pub fn clear_options_from_question(&mut self, section_id: Uuid, question_id: Uuid) {
        if let Some(section) = self.sections.iter_mut().find(|s| s.id == section_id) {
            if let Some(question) = section.questions.iter_mut().find(|q| q.id == question_id) {
                question.options.clear();
            }
        }
    }
}

impl QuestionSection {
    pub fn add_question(&mut self, question: Question) {
        self.questions.push(question)
    }

    pub fn remove_question(&mut self, question_id: Uuid) {
        self.questions.retain(|s| s.id != question_id)
    }

    pub fn update_question(&mut self, question: Question) {
        if let Some(pos) = self.questions.iter().position(|q| q.id == question.id) {
            self.questions[pos] = question
        }
    }
}

impl Default for QuestionSection {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            observation: String::new(),
            questions: Vec::new(),
        }
    }
}

impl Question {
    pub fn add_option(&mut self, option: AnswerOption) {
        self.options.push(option)
    }

    pub fn remove_option(&mut self, option_id: Uuid) {
        self.options.retain(|opt| opt.id != option_id)
    }

    pub fn update_option(&mut self, option: AnswerOption) {
        if let Some(pos) = self.options.iter().position(|o| o.id == option.id) {
            self.options[pos] = option
        }
    }
}

impl Default for Question {
    fn default() -> Self {
        Self { 
            id: Uuid::new_v4(),
            text: String::new(),
            question_type: QuestionType::SingleChoice,
            options: Vec::new(),
            points: 0,
        }
    }
}

impl AnswerOption {
    pub fn new(name: String) -> AnswerOption {
        AnswerOption { 
            id: Uuid::new_v4(), 
            text: name, 
            is_correct: false 
        }
    }
}


pub fn validate_all_questions_answered(quiz: &Quiz, answers: &Vec<UserAnswer>) -> bool {
    // We collect all the IDs of the questions of the maybe
    let all_question_ids: HashSet<Uuid> = quiz
        .sections
        .iter()
        .flat_map(|section| section.questions.iter())
        .map(|q| q.id)
        .collect();

    // We collect the IDS of the questions that have an answer
    let answered_question_ids: HashSet<Uuid> = answers.iter().map(|answer| match answer {
        UserAnswer::SingleChoice(q, _) => q.id,
        UserAnswer::MultipleChoice(q, _) => q.id,
        UserAnswer::TrueFalse(q, _, _) => q.id,
        UserAnswer::TextInput(q, _) => q.id,
    }).collect();

    // We compare if all questions have been answered
    if all_question_ids == answered_question_ids {
        // Ok(true)
        true
    } else {
        // let missing: Vec<_> = all_question_ids.difference(&answered_question_ids).collect();
        // Err(format!("Answers are missing for the following questions: {:?}", missing))
        false
    }
}