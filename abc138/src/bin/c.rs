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
        mut v:[f64;n],
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut k = v[0];
    for i in 1..n {
        k = (k + v[i]) / 2.;
    }
    println!("{}", k);
}
