use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis() as i64;
}

#[allow(dead_code)]
pub fn get_target_price(side: &i8, price: &f32, tpp: &f32) -> f32 {
    return if *side == 2 {
        *price + ((*price / 100 as f32) * *tpp)
    } else {
        *price - ((*price / 100 as f32) * *tpp)
    };
}