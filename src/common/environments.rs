#![allow(unused)]

use std::env;
use std::env::VarError;
use std::str::FromStr;

use log::error;

pub fn use_testnet() -> bool {
    let e = env::var("USE_TESTNET");
    map_to_boolean(e.unwrap().as_str())
}

fn map_to_boolean(e: &str) -> bool {
    match e {
        "true" => true,
        "t" => true,
        "false" => false,
        "f" => false,
        _ => false
    }
}