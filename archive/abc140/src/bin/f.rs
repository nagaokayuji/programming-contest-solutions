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
        mut a:[i64;1usize<<n],
    }
    a.sort();
    let mut s = vec![];
    let n2 = 1usize << n;
    s.push(*a.last().unwrap());
    a.pop();
    let mut ac = a.clone();
    // a[n2 - 1] = -1;
    for i in 0..n {
        let mut p = s.clone();
        p.sort();
        // let mut a = ac.clone();
        while a.len() > 0 && p.len() > 0 {
            if a.last().unwrap_or(&INF) < p.last().unwrap_or(&(-INF)) {
                s.push(*a.last().unwrap());
                p.pop();
            }
            a.pop();
        }
        if p.len() > 0 {
            yn(false);
            return;
        }
    }
    yn(true);
}
fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}

const INF: i64 = 1 << 60;
