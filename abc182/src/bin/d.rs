#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
    }
    let mut now = 0i64;
    let mut ans = 0i64;
    let mut cum = 0i64;
    let mut cummax = 0i64;
    for &x in a.iter() {
        ans = max(ans, now + cummax);
        now += cum + x;
        cum += x;
        cummax = max(cummax, cum);
        ans = max(ans, now);
    }
    println!("{}", ans);
}
