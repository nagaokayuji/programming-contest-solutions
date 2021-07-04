#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
#[fastout]
fn main() {
    input! {//
        n:usize,
        mut ab:[(i64,i64);n],
    }
    ab.sort_by_key(|&a| -(a.0 * 2 + a.1));
    let mut cnt = -ab.iter().fold(0, |sum, x| sum + x.0);
    for i in 0..n {
        cnt += 2 * ab[i].0 + ab[i].1;
        if cnt > 0 {
            println!("{}", i + 1);
            return;
        }
    }
}
