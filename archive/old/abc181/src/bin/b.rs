#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
           n:usize,
    mut       ab:[(i64,i64);n],
       }
    let sm = |x: i64| x * (x + 1) / 2;
    let mut ans = 0i64;
    for &(a, b) in ab.iter() {
        ans += sm(b) - sm(a - 1)
    }
    println!("{}", ans);
}
