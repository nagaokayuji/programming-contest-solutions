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
        n:usize,
        mut xy:[(f64,f64);n],
    }
    let pi = 3.141592653589793;
    let mut cnt = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            let xyi = xy[i];
            let xyj = xy[j];
            let dx = (xyi.0 - xyj.0).abs();
            let dy = (xyi.1 - xyj.1).abs();
            let tan = dy.atan2(dx);
            if -pi / 4. <= tan && tan <= pi / 4. {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
