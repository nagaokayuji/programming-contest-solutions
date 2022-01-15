#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,
        lr:[(Usize1,Usize1);m],
    }
    let mut lmax = 0;
    let mut rmax = n;
    for &(l, r) in lr.iter() {
        lmax = max(lmax, l);
        rmax = min(rmax, r + 1);
    }
    println!("{}", rmax.saturating_sub(lmax));
}
