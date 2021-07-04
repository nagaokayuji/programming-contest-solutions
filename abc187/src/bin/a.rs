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
        a:i64,b:i64,
    }
    println!("{}", max(dig(a), dig(b)));
}
fn dig(n: i64) -> i64 {
    let mut m = n;
    let mut ret = 0;
    while m > 0 {
        ret += m % 10;
        m /= 10;
    }
    ret
}
