#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut xy:[(i64,i64);n],
    }
    // x と t でわけてかんがえる
    xy.sort();

    let mut ans = 0i64;
    let mut xy45 = vec![];
    let mut xy135 = vec![];
    for &(x, y) in xy.iter() {
        xy45.push((x + y, x - y));
        xy135.push((x - y, x + y));
    }
    // dbg!(&xy45);
    xy45.sort();
    xy135.sort();
    let mut a1 = abs(xy45[0].0, xy45[n - 1].0);
    let mut a2 = abs(xy135[0].0, xy135[n - 1].0);
    println!("{}", max(a1, a2));
}
fn abs(i: i64, j: i64) -> i64 {
    return (i - j).abs();
}
