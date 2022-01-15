#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        l:i64,r:i64,
    }
    if r - l > 2019 {
        println!("{}", 0);
    } else {
        let mut ans = 2018;
        for i in l..=r {
            for j in i + 1..=r {
                ans = min(i * j % 2019, ans);
            }
        }
        println!("{}", ans);
    }
}
