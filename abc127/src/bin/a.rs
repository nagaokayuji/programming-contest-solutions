#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    a:usize,b:usize,
     }
    if a >= 13 {
        println!("{}", b);
    } else if a >= 6 {
        println!("{}", b / 2);
    } else {
        println!("{}", 0);
    }
}
