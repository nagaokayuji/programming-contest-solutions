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
        n:usize,
        a:[Usize1;n],
        b:[i64;n],
        c:[i64;n-1],
    }
    let mut ans = 0i64;
    for i in 0..n {
        ans += b[a[i]];
        if i > 0 && a[i - 1] + 1 == a[i] {
            ans += c[a[i - 1]];
        }
    }
    println!("{}", ans);
}
