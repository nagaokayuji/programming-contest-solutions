#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:i64,m:usize,
        a:[i64;m],
    }
    let mut ans = 0;
    let sm = a.iter().fold(0, |sm, x| sm + x);
    if n < sm {
        println!("{}", -1);
    } else {
        println!("{}", n - sm);
    }
}
