// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const BASE_RATE: f64 = 221.0;
pub fn production_rate_per_hour(speed: u8) -> f64 {
    return (speed as f64) * BASE_RATE as f64 * error_rate(speed);
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

fn error_rate(speed: u8) -> f64 {
    match speed {
        0..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!("unexpected speed")
    }
}