#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        a:i64,b:i64
    }
    for i in -200..201 {
        for j in -200..201 {
            if i + j == a && i - j == b {
                println!("{} {}", i, j);
                return;
            }
        }
    }
}

pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
