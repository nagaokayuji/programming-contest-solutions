#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        h:[i64;n],
    }
    let mut prev = -33;
    pub fn yn(ans: bool) {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    let mut ans = true;
    for i in 0..n {
        if prev > h[i] {
            if prev - 1 == h[i] {
                prev = h[i] + 1;
            } else {
                ans = false;
                break;
            }
        } else {
            prev = h[i];
        }
    }
    yn(ans);
}
