use uuid::Uuid;
use std::format;
use chrono::NaiveDateTime;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent, HtmlTextAreaElement};

use roboxmaker_models::user_model::user_by_id;
use roboxmaker_types::types::{MyUserProfile, DataSchoolProfile, UserId};

pub fn get_creation_date(datetime: NaiveDateTime) -> String {
    let timestamp = datetime;
    let now = chrono::Local::now().naive_local();
    let diff = now - timestamp;

    if diff.num_minutes() < 1 {
        return "Hace un momento".to_string();
    } else if diff.num_minutes() < 60 {
        return format!("Hace {} min", diff.num_minutes());
    } else if diff.num_hours() < 24 {
        return format!("Hace {}h", diff.num_hours());
    } else if diff.num_days() < 30 {
        if diff.num_days() == 1 {
            return "Hace 1 día".to_string();
        } else {
            return format!("Hace {} días", diff.num_days());
        }
    } else if diff.num_days() < 365 {
        if diff.num_days() < 60 {
            return "Hace 1 mes".to_string();
        } else {
            return format!("Hace {} meses", diff.num_days() / 30);
        }
    } else {
        if diff.num_days() / 365 == 1 {
            return "Hace 1 año".to_string();
        } else {
            return format!("Hace {} años", diff.num_days() / 365);
        }
    }
}


pub fn get_creation_date_robot(datetime: NaiveDateTime) -> String {
    let timestamp = datetime;
    let now = chrono::Local::now().naive_local();
    let diff = now - timestamp;

    if diff.num_minutes() < 1 {
        return "hace un momento".to_string();
    } else if diff.num_minutes() < 60 {
        return format!("hace {} min", diff.num_minutes());
    } else if diff.num_hours() < 24 {
        return format!("hace {}h", diff.num_hours());
    } else if diff.num_days() < 30 {
        if diff.num_days() == 1 {
            return "hace 1 día".to_string();
        } else {
            return format!("hace {} días", diff.num_days());
        }
    } else if diff.num_days() < 365 {
        if diff.num_days() < 60 {
            return "hace 1 mes".to_string();
        } else {
            return format!("hace {} meses", diff.num_days() / 30);
        }
    } else {
        if diff.num_days() / 365 == 1 {
            return "hace 1 año".to_string();
        } else {
            return format!("hace {} años", diff.num_days() / 365);
        }
    }
}

pub fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.clone().dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());

    target.value()
}

pub fn get_value_from_textarea_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    // web_sys::console::log_1(&target.value().into());
    target.value()
}

pub fn user_profile_data() -> Option<MyUserProfile> {
    let user_data: Option<MyUserProfile> = LocalStorage::get("USER-PROFILE").ok();
    user_data
}

pub fn school_profile_data() -> Option<DataSchoolProfile> {
    let user_data: Option<DataSchoolProfile> = LocalStorage::get("SCHOOL-PROFILE").ok();
    user_data
}


pub fn menu_state() {
    gloo_storage::LocalStorage::set("home-state", false).ok();
    gloo_storage::LocalStorage::set("school-state", false).ok();
    gloo_storage::LocalStorage::set("myspace-state", false).ok();
    gloo_storage::LocalStorage::set("meets-state", false).ok();
}

pub fn home_state() {
    gloo_storage::LocalStorage::set("home-state", true).ok();
    gloo_storage::LocalStorage::set("school-state", false).ok();
    gloo_storage::LocalStorage::set("myspace-state", false).ok();
    gloo_storage::LocalStorage::set("meets-state", false).ok();
}

pub fn school_state() {
    gloo_storage::LocalStorage::set("home-state", false).ok();
    gloo_storage::LocalStorage::set("school-state", true).ok();
    gloo_storage::LocalStorage::set("myspace-state", false).ok();
    gloo_storage::LocalStorage::set("meets-state", false).ok();
}

pub fn myspace_state() {
    gloo_storage::LocalStorage::set("home-state", false).ok();
    gloo_storage::LocalStorage::set("school-state", false).ok();
    gloo_storage::LocalStorage::set("myspace-state", true).ok();
    gloo_storage::LocalStorage::set("meets-state", false).ok();
}

pub fn meets_state() {
    gloo_storage::LocalStorage::set("home-state", false).ok();
    gloo_storage::LocalStorage::set("school-state", false).ok();
    gloo_storage::LocalStorage::set("myspace-state", false).ok();
    gloo_storage::LocalStorage::set("meets-state", true).ok();
}


pub fn user_profile(data: Option<user_by_id::ResponseData>) -> Option<MyUserProfile> {
    let user_data = data
        .clone()
        .and_then(|user| user.user_by_pk)
        .and_then(|item| {
            Some(MyUserProfile {
                email: item.user_profile.clone().and_then(|d|d.email).unwrap_or("example.123@gmail.co".to_string()),
                full_name: item.user_profile.clone().and_then(|d|Some(d.full_name)).unwrap_or("".to_string()),
                pic_path: item.user_profile.clone().and_then(|d|d.pic_path).unwrap_or("/static/avatar.png".to_string()),
                user_id: roboxmaker_types::types::UserId(item.user_profile.clone().and_then(|d|Some(d.user_id)).unwrap_or(Uuid::default())),
                school_name: item.user_profile.clone().and_then(|d|d.group_member).and_then(|group_member|group_member.school_group).and_then(|school_group|Some(school_group.school)).and_then(|school|school.school_profile).and_then(|school_profile|Some(school_profile.name)).unwrap_or("".to_string()),
                user_student: item.user_student.and_then(|d|Some(UserId(d.user_id))),
                user_teacher: item.user_teacher.and_then(|d|Some(UserId(d.user_id))),
                user_staff: item.user_staff.and_then(|d|Some(UserId(d.user_id))),
                license: item.license.and_then(|d|Some(d.license)).unwrap_or("AAAAAAAAAAAAAAA".to_string()),
                group_member_id: roboxmaker_types::types::GroupId(item.group_member.and_then(|d|Some(d.group_id)).unwrap_or(Uuid::default())),
            })
        });
        
    user_data
}

// pub fn return_user_profile() -> Option<MyUserProfile> {
//     let data: Option<user_by_id::ResponseData> = Default::default();
//     let user_data = data
//         .clone()
//         .and_then(|user| user.user_by_pk)
//         .and_then(|item| {
//             Some(MyUserProfile {
//                 email: item.user_profile.clone().and_then(|d|d.email).unwrap_or("example.123@gmail.co".to_string()),
//                 full_name: item.user_profile.clone().and_then(|d|Some(d.full_name)).unwrap_or("".to_string()),
//                 pic_path: item.user_profile.clone().and_then(|d|d.pic_path).unwrap_or("/static/avatar.png".to_string()),
//                 user_id: roboxmaker_types::types::UserId(item.user_profile.clone().and_then(|d|Some(d.user_id)).unwrap_or(Uuid::default())),
//                 school_name: item.user_profile.clone().and_then(|d|d.group_member).and_then(|group_member|group_member.school_group).and_then(|school_group|Some(school_group.school)).and_then(|school|school.school_profile).and_then(|school_profile|Some(school_profile.name)).unwrap_or("".to_string()),
//                 user_student: item.user_student.and_then(|d|Some(UserId(d.user_id))),
//                 user_teacher: item.user_teacher.and_then(|d|Some(UserId(d.user_id))),
//                 user_staff: item.user_staff.and_then(|d|Some(UserId(d.user_id))),
//                 license: item.license.and_then(|d|Some(d.license)).unwrap_or("AAAAAAAAAAAAAAA".to_string()),
//                 group_member_id: roboxmaker_types::types::GroupId(item.group_member.and_then(|d|Some(d.group_id)).unwrap_or(Uuid::default())),
//             })
//         });
        
//     user_data
// }