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
        n:i64
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j == n {
                yn(true);
                return;
            }
        }
    }
    yn(false);
}
