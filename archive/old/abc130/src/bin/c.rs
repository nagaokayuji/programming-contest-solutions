#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        w:i64,h:i64,x:i64,y:i64,
    }
    if x * 2 == w && y * 2 == h {
        println!("{} {}", (w * h) as f64 / 2.0, 1);
    } else {
        println!("{} {}", (w * h) as f64 / 2.0, 0);
    }
}
