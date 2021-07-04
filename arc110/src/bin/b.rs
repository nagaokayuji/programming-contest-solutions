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
        mut s:Chars,
    }
    if n == 1 {
        if s[0] == '1' {
            println!("{}", 1e10 as i64 * 2);
        } else {
            println!("{}", 1e10 as i64);
        }
        return;
    }
    if n == 2 {
        if s[0] == '1' && s[1] == '1' {
            println!("{}", 1e10 as i64);
        } else if s[0] == '1' && s[1] == '0' {
            println!("{}", 1e10 as i64);
        } else if s[0] == '0' && s[1] == '1' {
            println!("{}", 1e10 as i64 - 1);
        } else {
            println!("{}", 0);
        }
        return;
    }
    let mut t0 = true;
    let mut t1 = true;
    let mut t2 = true;
    let mut bas = vec!['1', '1', '0'];
    for i in 0..n {
        if bas[i % 3] != s[i] {
            t0 = false;
        }
        if bas[(i + 1) % 3] != s[i] {
            t1 = false;
        }
        if bas[(i + 2) % 3] != s[i] {
            t2 = false;
        }
    }
    // dbg!((t0, t1, t2));
    if (t0 || t1 || t2) == false {
        println!("{}", 0);
        return;
    }
    let mut mult = 1e10 as usize;
    if t0 {
        let us = (n + 2) / 3;
        println!("{}", mult - us + 1);
    } else if t1 {
        let us = (n + 3) / 3;
        println!("{}", mult - us + 1);
    } else {
        let us = (n + 4) / 3;
        println!("{}", mult - us + 1);
    }
}
