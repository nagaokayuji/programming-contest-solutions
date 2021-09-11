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
    let c = a + b;
    if c >= 15 && b >= 8 {
        println!("{}", 1);
    } else if c >= 10 && b >= 3 {
        println!("{}", 2);
    } else if c >= 3 {
        println!("{}", 3);
    } else {
        println!("{}", 4);
    }
}
