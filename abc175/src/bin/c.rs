#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        x:i64,
        k:i64,
        d:i64,
    }
    let x = x.abs() as i64;

    // d*i >= k naruka?
    // i >= k/d
    // k/d
    if k < x / d {
        println!("{}", x - d * k);
        return;
    }
    let mut ans = INF;
    // d*k >= x のとき
    // 解の候補　d*k-x, x-d*k
    let c = x / d;
    if c % 2 == k % 2 {
        ans = x - c * d;
    } else {
        ans = (x - (c + 1) * d).abs();
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
