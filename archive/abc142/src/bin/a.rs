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
        n:usize,
    }

    let mut oddc = (n + 1) / 2;
    println!("{}", oddc as f64 / n as f64);
}
const MX: usize = 1010101;
