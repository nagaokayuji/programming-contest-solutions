#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        l:[i64;n],
    }
    let mut ans = 0i64;
    for i in 0..n {
        for j in i + 1..n {
            if l[i] == l[j] {
                continue;
            }
            for k in j + 1..n {
                if l[i] == l[k] || l[j] == l[k] {
                    continue;
                }
                let mut v = vec![l[i], l[j], l[k]];
                v.sort();
                if v[2] < v[0] + v[1] {
                    ans += 1i64;
                }
            }
        }
    }
    println!("{}", ans);
}
