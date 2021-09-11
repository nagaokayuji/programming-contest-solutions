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
        mut a:[i64;n],
    }
    let mut s = a.iter().fold(0, |sum, x| sum ^ x);
    let mut ans = s;
    for i in 0..n {
        a[i] &= !s;
    }
    // 掃き出し法
    let mut rank = 0;
    let mut top = vec![0; n];
    for i in (0..=60).rev() {
        let mut j = rank;
        while j < n {
            if a[j] >> i & 1 == 1 {
                a.swap(rank, j);
                break;
            }
            j += 1;
        }
        // not found
        if j == n {
            continue;
        }
        top[rank] = i;
        for k in 0..n {
            if rank == k {
                continue;
            }
            if a[k] >> i & 1 == 1 {
                a[k] ^= a[rank];
            }
        }
        rank += 1;
    }
    let x = a.iter().fold(0, |sum, x| sum ^ x);
    ans += x * 2;
    println!("{}", ans);
}
