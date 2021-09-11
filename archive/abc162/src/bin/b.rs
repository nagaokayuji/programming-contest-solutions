#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:i64,
    }
    let mut ans = 0;
    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            ans += i;
        }
    }
    println!("{}", ans);
}
