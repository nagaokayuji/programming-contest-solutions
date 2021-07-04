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
        h:usize,w:usize,
        a:[[i64;w];h]
    }
    let mut ans = INF;
    for k in 0..=100 {
        let mut t = 0;
        for i in 0..h {
            for j in 0..w {
                if a[i][j] < k {
                    t = INF;
                }
                t += (a[i][j] - k);
            }
        }
        ans = min(ans, t);
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
