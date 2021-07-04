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
        n:usize,m:i64,
        a:[i64;n],
    }
    let mut b = a.iter().map(|&x| x / 2).collect::<Vec<_>>();
    let c = a
        .iter()
        .map(|&x| {
            let mut c = 0i64;
            let mut x = x;
            while x % 2 == 0 {
                x /= 2;
                c += 1
            }
            c
        })
        .collect::<Vec<_>>();
    if !c.iter().all(|&x| x == *c.first().unwrap()) {
        println!("{}", 0);
        return;
    }
    let l = lcm_list(&b);
    println!("{}", (m / l + 1) / 2);
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm_list(list: &[i64]) -> i64 {
    list.iter().fold(list[0], |a, &b| lcm(a, b))
}
