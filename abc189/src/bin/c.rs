#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
    }
    let mut ans = 0i64;
    let mut height = INF;
    for l in 0..n {
        height = a[l];
        for r in l..n {
            // [l,r]
            height = min(height, a[r]);
            ans = max(ans, (r - l + 1) as i64 * height);
        }
        //
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
