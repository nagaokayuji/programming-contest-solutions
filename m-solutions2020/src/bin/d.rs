#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
    }
    a.push(-1);
    let mut pairs = vec![];
    let mut small = 9999;
    let mut big = 0;
    for &x in a.iter() {
        if big > x {
            pairs.push((small, big));
            small = 9999;
            big = 0;
        }
        small = min(small, x);
        big = max(big, x);
    }
    println!(
        "{}",
        pairs
            .iter()
            .fold(1000, |sum, x| sum + (x.1 - x.0) * (sum / x.0))
    );
}
