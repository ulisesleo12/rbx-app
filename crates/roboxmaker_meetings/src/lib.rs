pub mod button_create_meetings;
pub mod create_meet;
pub mod meetings_view;
pub mod select_option_degree;
pub mod meetings_list_home;
pub mod meet_room;
pub mod meet_session;
pub mod direct_meet_room;
pub mod direct_meet_session;
mod list_meetings_by_school;
mod last_meetings;
pub mod create_meeting_node;

use roboxmaker_types::types::{SchoolId, GroupId, ClassGroupMeetings};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMeet {
    pub class_name: String,
    pub group_id: GroupId,
    pub meetings: Vec<ClassGroupMeetings>,
    pub school_name: String,
    pub show_options: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMeetTwo {
    pub number_meet: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassGroupMeetData {
    pub class_name: String,
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub meetings: Vec<ClassGroupMeetings>,
}
