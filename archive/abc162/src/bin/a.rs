#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {

    mut       n:i64,
       }
    let mut ans = false;
    while n > 0 {
        if n % 10 == 7 {
            ans = true;
        }
        n /= 10;
    }
    yn(ans);
}

pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
