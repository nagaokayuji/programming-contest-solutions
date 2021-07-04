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
        mut a: [i64;n],
    }
    a.sort();
    let mut aa = a.iter().map(|&x| x * x).collect::<Vec<_>>();
    let mut saa = vec![0; n];
    saa[0] = aa[0];
    let mut sa = vec![0; n];
    sa[0] = a[0];
    for i in 1..n {
        saa[i] = saa[i - 1] + aa[i];
        sa[i] = sa[i - 1] + a[i];
    }
    let mut ans = 0i64;
    for i in 1..n {
        ans += a[i] * a[i] * i as i64;
        ans += saa[i - 1];
        ans -= 2 * a[i] * sa[i - 1];
    }
    println!("{}", ans);
}
