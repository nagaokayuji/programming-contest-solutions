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
        xy:[(i64,i64);n],
    }
    let mut edges = vec![];
    for i in 0..n {
        for j in i + 1..n {
            let dist = (xy[i].0 - xy[j].0).pow(2) + (xy[i].1 - xy[j].1).pow(2);
            edges.push((dist as f64).sqrt());
        }
    }
    let mut mul = 1;
    for i in 1..n {
        mul *= i;
    }
    println!(
        "{}",
        edges.iter().fold(0f64, |s, x| s + x) * (2f64) / n as f64
    );
}
