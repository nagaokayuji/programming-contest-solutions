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
        n:usize,k:usize,
        rsp:[i64;3],
        t:Chars,
    }
    let mut wn = vec![0; n];
    for i in 0..n {
        if t[i] == 'r' {
            wn[i] = 2;
        } else if t[i] == 's' {
            wn[i] = 0;
        } else {
            wn[i] = 1;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if i >= k && wn[i - k] == wn[i] {
            wn[i] = 1234;
        } else {
            ans += rsp[wn[i]];
        }
    }
    println!("{}", ans);
}
