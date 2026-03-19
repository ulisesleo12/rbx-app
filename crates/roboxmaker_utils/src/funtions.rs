use std::format;
use chrono::NaiveDateTime;
use wasm_bindgen::prelude::wasm_bindgen;

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

#[wasm_bindgen(module = "/src/funtions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "isMobileDevice")]
    fn is_mobile_device_js() -> bool;
}

pub fn is_mobile_device() -> bool {
    #[allow(unused_unsafe)]
    unsafe {
        is_mobile_device_js()
    }
}