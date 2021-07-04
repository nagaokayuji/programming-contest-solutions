#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        p:usize,q:usize,r:usize
    }
    println!("{}", p + q + r - max(max(p, q), r));
}
