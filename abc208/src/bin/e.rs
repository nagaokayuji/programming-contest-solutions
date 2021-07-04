#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        mut n:i64;
        k:i64;
    }
    let mut a = vec![];
    while n > 0 {
        a.push(n % 10);
        n /= 10;
    }
    dbg!(&a);
}

const INF: i64 = 1 << 59;
