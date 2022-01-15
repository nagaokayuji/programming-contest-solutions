#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:i64,
    }

    let mut ret = 0i64;
    for i in 1..90 {
        for j in 1..80 {
            if cal(i, j) == n {
                println!("{} {}", i, j);
                return;
            }
            if cal(i, j) == i64::MAX {
                break;
            }
        }
    }
    println!("{}", -1);
}
fn cal(three: usize, five: usize) -> i64 {
    let mut ret = 1i64;
    for i in 0..three {
        ret = ret.saturating_mul(3);
    }
    let mut ret2 = 1i64;
    for j in 0..five {
        ret2 = ret2.saturating_mul(5);
    }
    ret.saturating_add(ret2)
}
