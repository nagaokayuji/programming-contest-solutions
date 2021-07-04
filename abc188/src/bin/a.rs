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
        x:i64,y:i64
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
    yn((x - y).abs() < 3)
}
