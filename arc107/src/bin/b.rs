#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:i64,
        k:i64
    }

    // a+b = x, c+d = y => x-y = k
    // x-y=k

    let mut ans = 0i64;
    for x in 2..=(2 * n) {
        let y = x - k;
        // x-y=k
        if !(2..=(2 * n)).contains(&y) {
            continue;
        }
        ans += cal(x, n) * cal(y, n);
    }
    println!("{}", ans);
}
fn cal(x: i64, n: i64) -> i64 {
    //
    if n * 2 < x {
        0
    } else if n * 2 == x {
        1
    } else if x == 2 {
        1
    } else if x < 2 {
        0
    } else if n >= x - 1 {
        x - 1
    } else {
        let mut ok = n;
        let mut ng = 0;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if 1 <= mid && mid <= n && 1 <= x - mid && x - mid <= n {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        n - ok + 1
    }
}
