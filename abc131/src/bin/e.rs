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
        n:usize,k:usize,
    }

    if k > (n - 1) * (n - 2) / 2 {
        println!("{}", -1);
        return;
    }
    let mut edgs = vec![];
    for i in 1..n {
        edgs.push((0, i));
    }
    let mut now = (n - 1) * (n - 2) / 2;
    for i in 1..n {
        for j in i + 1..n {
            edgs.push((i, j));
        }
    }
    for _ in 0..k {
        edgs.pop();
    }
    println!("{}", edgs.len());
    for &(a, b) in edgs.iter() {
        println!("{} {}", a + 1, b + 1);
    }
}
