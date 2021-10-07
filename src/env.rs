use std::env;

use log::error;

pub fn get_base_url() -> String {
    let e = env::var("BASE_URL");
    if !env::var("BASE_URL").is_ok() {
        error!("BASE_URL is empty");
    }
    return e.unwrap();
}

pub fn get_access_id() -> String {
    let e = env::var("ACCESS_ID");
    if !env::var("ACCESS_ID").is_ok() {
        error!("ACCESS_ID is empty");
    }
    return e.unwrap();
}

pub fn get_secret_key() -> String {
    let e = env::var("SECRET_KEY");
    if !env::var("SECRET_KEY").is_ok() {
        error!("SECRET_KEY is empty");
    }
    return e.unwrap();
}

pub fn get_position_type() -> i8 {
    let e = env::var("POSITION_TYPE");
    if !env::var("POSITION_TYPE").is_ok() {
        error!("POSITION_TYPE is empty");
    }
    return e.unwrap().parse::<i8>().unwrap();
}

pub fn get_tpp() -> f32 {
    let e = env::var("TPP");
    if !env::var("TPP").is_ok() {
        error!("TPP is empty");
    }
    return e.unwrap().parse::<f32>().unwrap();
}

pub fn get_initial_balance() -> i32 {
    let e = env::var("INITIAL_BALANCE");
    if !env::var("INITIAL_BALANCE").is_ok() {
        error!("INITIAL_BALANCE is empty");
    }
    return e.unwrap().parse::<i32>().unwrap();
}

pub fn get_concurrent_position() -> i32 {
    let e = env::var("CONCURRENT_POSITION");
    if !env::var("CONCURRENT_POSITION").is_ok() {
        error!("CONCURRENT_POSITION is empty");
    }
    return e.unwrap().parse::<i32>().unwrap();
}

pub fn get_leverage() -> i32 {
    let e = env::var("LEVERAGE");
    if !env::var("LEVERAGE").is_ok() {
        error!("LEVERAGE is empty");
    }
    return e.unwrap().parse::<i32>().unwrap();
}