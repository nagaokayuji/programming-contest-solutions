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
        a:i64,b:i64,c:i64,d:i64,
    }
    println!("{}", f(b, c, d) - f(a - 1, c, d));
}
fn f(x: i64, c: i64, d: i64) -> i64 {
    let lcm = lcm(c, d);
    x - x / c - x / d + x / lcm
}
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
