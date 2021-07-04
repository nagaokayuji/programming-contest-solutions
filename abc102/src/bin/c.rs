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

    for i in 0..n {
        a[i] -= (i as i64 + 1);
    }

    a.sort();
    let mut k = 0;
    if n % 2 == 0 {
        k = (a[n / 2] + a[n / 2 - 1]) / 2;
    } else {
        k = a[n / 2];
    }
    let mut ans = 0;
    for i in 0..n {
        ans += (a[i] - k).abs();
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
