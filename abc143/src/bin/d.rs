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
        mut L:[i64;n],
    }

    L.sort();
    let mut ans = 0;
    for b in 0..n {
        let mut r = b + 1;
        for a in 0..b {
            let ab = L[a] + L[b];
            while r < n && L[r] < ab {
                r += 1;
            }
            let l = b + 1;
            if r > l {
                ans += r - l;
            }
        }
    }
    println!("{}", ans);
}
