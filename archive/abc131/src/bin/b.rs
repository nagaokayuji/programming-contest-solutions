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
        n:usize,l:i64,
    }
    let all = (0..n).fold(0, |sum, x| sum + x as i64 + l);
    let mn = (0..n).map(|x| ((x as i64 + l).abs(), x)).min().unwrap();
    println!("{}", all - (mn.1 as i64 + l));
}
