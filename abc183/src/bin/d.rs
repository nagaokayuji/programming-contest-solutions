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
        w:i64,
        mut stp:[(usize,usize,i64);n],
    }
    stp.sort();
    let mut imo = vec![0i64; MX];
    for &(s, t, p) in stp.iter() {
        imo[s] += p;
        imo[t] -= p;
    }
    // dbg!(&imo[0..11]);
    let mut now = 0;
    for i in 0..MX {
        now += imo[i];
        if now > w {
            yn(false);
            return;
        }
    }
    yn(true);
}

fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}

const MX: usize = 410101;
