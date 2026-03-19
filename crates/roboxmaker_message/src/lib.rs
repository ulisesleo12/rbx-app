use uuid::Uuid;
use serde::Deserialize;
use roboxmaker_types::types::{LessonId, PostId, GroupId, RobotId};

pub mod message_card;
pub mod message_list;
pub mod list_responses;
pub mod user_messages;
pub mod reply_message;
pub mod response_message;
// pub mod message_list_post;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageGroupCategory {
    Posts(GroupId, PostId),
    Robots(GroupId, RobotId),
    Lessons(GroupId, LessonId),
    DirectMessages(GroupId),
    FilesUser,
}


#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MessageAuthor {
    pub user_id: Uuid,
    pub full_name: String,
    pub pic_path: String,
    pub user_staff: Option<Uuid>,
    pub user_teacher: Option<Uuid>,
    pub user_student: Option<Uuid>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct MessageProfile {
    pub message_id: Uuid,
    pub author: MessageAuthor,
    pub content: String,
    pub timestamp: String,
    pub reply_id: Option<Uuid>,
    pub replies_private: bool,
}