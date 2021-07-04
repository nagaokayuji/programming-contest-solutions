#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,x:i64,
        mut vp:[(i64,i64);n],
    }
    let mut sum = 0i64;

    for i in 0..n {
        sum += vp[i].0 * vp[i].1;
        if sum > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}
