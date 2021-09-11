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
        n:usize,k:usize,m:usize,
        a:[usize;n-1],
    }
    let sm = a.iter().sum::<usize>();
    // sm + x = m*n
    let x = (m * n) as i64 - sm as i64;
    if x > k as i64 {
        println!("{}", -1);
    } else {
        println!("{}", max(0, x));
    }
}
