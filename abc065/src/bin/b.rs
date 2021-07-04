#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut a:[Usize1;n],
    }
    // 0-> a[0] -> a[a[0]] -> ... -> 1 かどうか

    let mut now = 0;
    for cnt in 0..n * 2 {
        if now == 1 {
            println!("{}", cnt);
            return;
        }
        now = a[now];
    }
    println!("{}", -1);
}
