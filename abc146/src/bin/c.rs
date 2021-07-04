#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        a:i64,b:i64,x:i64,
    }
    let mut ok = 0i64;
    let mut ng = INF;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let isOk = || a * mid + b * dig(mid) <= x;
        if isOk() {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", min(ok, 1000000000));
}
const INF: i64 = 1 << 34;

fn dig(n: i64) -> i64 {
    let mut m = n;
    let mut ret = 0;
    while m > 0 {
        m /= 10;
        ret += 1;
    }
    ret
}
