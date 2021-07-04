#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize
    }
    let mut ans = 0;
    for i in 1..=n {
        if f(i) & 1 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn f(n: usize) -> usize {
    let mut ret = 0;
    let mut n = n;
    while n > 0 {
        ret += 1;
        n /= 10;
    }
    ret
}
