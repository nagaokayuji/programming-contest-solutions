#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        k:i64,
    }
    let mut ans = 0;
    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                ans += gcd_list(&vec![a, b, c]);
            }
        }
    }
    println!("{}", ans);
}
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn gcd_list(list: &[i64]) -> i64 {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}
