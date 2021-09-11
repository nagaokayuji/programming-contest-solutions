#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        mut a:usize,mut b:usize,mut c:usize,
        k:usize,
    }
    for _ in 0..k {
        if b <= a {
            b <<= 1;
        } else if c <= b {
            c <<= 1;
        }
    }
    if (a + 1..c).contains(&b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
