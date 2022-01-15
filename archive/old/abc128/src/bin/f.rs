#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        s:[i64;n],
    }
    let mut res = 0;
    for p in 1..n {
        let mut tmp = 0;
        let mut cur = 0;
        let mut l = 0;
        let mut r = n - 1;
        if (n - 1) % p == 0 {
            while l < r {
                cur += s[l] + s[r];
                tmp = max(tmp, cur);
                l += p;
                r -= p;
            }
        } else {
            while l < n - 1 && r > p {
                cur += s[l] + s[r];
                tmp = max(tmp, cur);
                l += p;
                r -= p;
            }
        }
        res = max(res, tmp);
    }
    println!("{}", res);
}
