#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,d:i64,
      mut  xy:[(i64,i64);n],
    }
    let mut ans = 0;
    for &(x, y) in xy.iter() {
        if x * x + y * y <= d * d {
            ans += 1;
        }
    }

    println!("{}", ans);
}
