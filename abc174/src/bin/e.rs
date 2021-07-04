#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:i64,
        mut a:[i64;n],
    }
    let mut ng = 0;
    let mut ok = INF;

    while (ok - ng).abs() > 1 {
        let mid = (ng + ok) >> 1;

        let check = |mid: i64| -> bool {
            let mut cnt = 0;
            for &x in a.iter() {
                cnt += (x + mid - 1) / mid - 1;
            }
            cnt <= k
        };
        if check(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
const INF: i64 = 1 << 60;
