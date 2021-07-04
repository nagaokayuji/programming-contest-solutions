#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,
    }

    let mut ans = 0i64;
    for i in 1..=n {
        let mut k = i;
        let mut ten = true;
        while k > 0 {
            if k % 10 == 7 {
                ten = false;
                break;
            }
            k /= 10;
        }
        k = i;
        let mut ei = true;
        while k > 0 {
            if k % 8 == 7 {
                ei = false;
                break;
            }
            k /= 8;
        }
        if ten && ei {
            ans += 1;
        }
    }
    println!("{}", ans);
}
fn f(n: usize) {
    let mut n = n;
}
