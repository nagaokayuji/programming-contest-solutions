#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut nx = 0;

    let mut ans = 0;
    for &x in a.iter() {
        if x == nx + 1 {
            nx = x;
        }
    }
    if nx == 0 {
        println!("{}", -1);
    } else {
        println!("{}", n - nx);
    }
}
