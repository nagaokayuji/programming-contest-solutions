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
        n:usize,
        a:[i64;1<<n],
    }
    let mut na = vec![];
    for i in 0..1 << n {
        na.push((a[i], i + 1));
    }
    if n == 1 {
        if na[0].0 < na[1].0 {
            println!("{}", na[0].1);
        } else {
            println!("{}", na[1].1);
        }
        return;
    }
    let mut pa = na.clone();
    for i in 1..n {
        let mut na = vec![];
        for j in 0..(1 << n - i) {
            if pa[2 * j].0 < pa[2 * j + 1].0 {
                na.push(pa[2 * j + 1]);
            } else {
                na.push(pa[2 * j]);
            }
        }
        if i == n - 1 {
            assert!(na.len() == 2);
            if na[0].0 < na[1].0 {
                println!("{}", na[0].1);
            } else {
                println!("{}", na[1].1);
            }
            return;
        }
        pa = na.clone();
    }
}
