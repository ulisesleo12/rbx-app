pub mod activity_card;
pub mod activity_list;
pub mod v_calendar_activity;
pub mod activity_card_classes;

use roboxmaker_types::types::{ClassesId, GroupId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivityGroupCategory {
    Classes(GroupId, ClassesId),
    ClassesView(GroupId, ClassesId),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivityStyle {
    ClassesPage,
    ClassesCard,
}