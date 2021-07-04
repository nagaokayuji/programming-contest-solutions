#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut x:[i64;n],
    }
    let mut mx = x.iter().map(|x| x.abs()).max().unwrap();
    let mut man = x.iter().fold(0, |sum, x| sum + x.abs());

    println!("{}", man);

    let mut ssum = x.iter().fold(0, |sum, x| sum + x * x);
    let ss = ssum as f64;
    println!("{}", ss.sqrt());

    println!("{}", mx);
}
