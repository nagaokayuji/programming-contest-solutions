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
        a:f64,b:f64,x:f64,
    }
    let pi = (-1f64).acos();

    let mut ok = 0f64;
    let mut ng = pi / 2f64;
    for _ in 0..500 {
        let mid = (ok + ng) / 2f64;
        let calc = |t: f64| {
            if a * t.tan() < b {
                a * a * b - a * a * a * t.tan() / 2f64
            } else {
                b * b / t.tan() * a / 2f64
            }
        };
        if calc(mid) > x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok * 180f64 / pi);
}
