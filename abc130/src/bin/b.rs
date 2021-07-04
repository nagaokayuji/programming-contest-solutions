#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,x:usize,
        l:[usize;n],
    }
    let mut ans = 1; // d[0]
    let mut d = 0;
    for i in 0..n {
        d += l[i];
        if d <= x {
            ans += 1;
        }
    }
    println!("{}", ans);
}
