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
        a:[i64;n],
        b:[i64;n],
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
    yn(a.iter().zip(b.iter()).fold(0, |sum, x| sum + x.0 * x.1) == 0);
}
