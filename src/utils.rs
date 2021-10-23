use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

use log::warn;

use crate::env::get_base_url;

pub fn get_current_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis() as i64;
}

#[allow(dead_code)]
pub fn get_target_price(side: &str, price: &f32, tpp: &f32) -> f32 {
    return if side.eq("LONG") {
        *price + ((*price / 100 as f32) * *tpp)
    } else {
        *price - ((*price / 100 as f32) * *tpp)
    };
}

pub fn validate_url(target_path: String) -> String {
    let base_url = get_base_url();
    if !base_url.starts_with("https") {
        warn!("Wrong base url, please fix: {}\n", base_url);
        process::exit(1);
    }

    return if base_url.ends_with("/") {
        format!("{}{}", base_url, target_path)
    } else {
        format!("{}/{}", base_url, target_path)
    };
}