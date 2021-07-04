#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,t:usize,s:usize
    }
    yn(n <= t * s);
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
