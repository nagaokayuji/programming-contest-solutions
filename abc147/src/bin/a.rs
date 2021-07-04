#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        a:[i64;3],
    }
    println!(
        "{}",
        if a.iter().sum::<i64>() >= 22 {
            "bust"
        } else {
            "win"
        }
    );
}
