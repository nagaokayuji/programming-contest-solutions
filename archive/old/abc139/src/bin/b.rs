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
        a:usize,
        b:usize,
    }
    let mut now = 1;
    let mut ret = 0;
    while now < b {
        ret += 1;
        now += a - 1;
    }
    println!("{}", ret);
}
