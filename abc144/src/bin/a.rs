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
        a:i64,b:i64,
    }
    if a < 10 && b < 10 {
        println!("{}", a * b);
    } else {
        println!("{}", -1);
    }
}
