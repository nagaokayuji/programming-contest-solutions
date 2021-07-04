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
        n:i64,
    }
    let l = n % 100;
    let u = (n - l) / 100;
    dbg!((u, l));
    if (1..=12).contains(&u) {
        if (1..=12).contains(&l) {
            println!("AMBIGUOUS");
        } else {
            println!("MMYY");
        }
    } else {
        if (1..=12).contains(&l) {
            println!("YYMM");
        } else {
            println!("NA");
        }
    }
}
