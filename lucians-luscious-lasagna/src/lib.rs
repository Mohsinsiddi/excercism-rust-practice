// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::num;

pub fn expected_minutes_in_oven() -> i32 {
    return 40;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    return  actual_minutes_in_oven-10;;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    return  2* number_of_layers;
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return number_of_layers*2+actual_minutes_in_oven;
}
