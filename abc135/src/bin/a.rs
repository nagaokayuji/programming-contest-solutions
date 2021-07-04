#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
       mut a:i32,mut b:i32,
    }
    if (b + a) % 2 != 0 {
        println!("IMPOSSIBLE");
    } else {
        println!("{}", (b + a) / 2);
    }
}
