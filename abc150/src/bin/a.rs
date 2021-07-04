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
        k:i64,x:i64,
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
    yn(k * 500 >= x);
}
