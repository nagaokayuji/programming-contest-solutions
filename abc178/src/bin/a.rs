#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
    a:i64,
        }
    println!("{}", if a == 0 { 1 } else { 0 });
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
