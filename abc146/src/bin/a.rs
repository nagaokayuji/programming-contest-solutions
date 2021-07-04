#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        s:String,
    }
    let k = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    println!("{}", 7 - k.iter().position(|&x| &x == &&s).unwrap());
}
