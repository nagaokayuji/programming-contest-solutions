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
        n:usize,x:i64,
        mut a:[i64;n],
    }
    let k = a.iter().filter(|&&y| y != x).collect::<Vec<_>>();
    if k.len() == 0 {
        println!("");
    } else {
        for x in k {
            println!("{}", x);
        }
    }
}
