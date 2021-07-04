#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
    n:usize,m:usize,
    mut w:[i64;n],
    mut lv:[(i64,i64);m],
    }
    w.sort();
    let mut mxv = lv.iter().map(|&x| x.1).max().unwrap();
    let mut mxcamel = w[n - 1];
    dbg!(mxv);
    dbg!(mxcamel);
    if mxcamel > mxv {
        println!("{}", -1);
        return;
    }

    // // 初期値
    // let mut ans = lv.iter().map(|&x| x.0).fold(0, |sum, x| sum + x);
    // //しゃく

    // let mut l = 0;
    // let mut r = 0;
    // let mut len = 0;
    // while l < m {

    // }
    w.reverse();
    let mut j = 0;
    let mut len = 0;
    let mut ans = 0;

    let mut sum = 0;

    let mut l = 0;
    let mut nowlen = 0;
    while l < m {
        // わたる
        while j < n && sum + w[j] <= lv[l].1 {
            sum += w[j];
            j += 1;
        }
        // みんなわたった
        if j == n {
            ans = max(ans, nowlen);
        }
    }

    println!("{}", ans);
}
