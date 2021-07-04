#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        a:[f64;n],
    }
    //    mut  a:[f64;n],

    let mut b = vec![0i64; n];
    for i in 0..n {
        b[i] = (a[i] * 1e9 + 0.5f64) as i64;
    }
    let mut c = vec![];
    let mut tfs = vec![vec![0i64; 20]; 20];
    for &x in b.iter() {
        let mut two = 0;
        let mut five = 0;
        let mut y = x;
        while y % 2 == 0 {
            y /= 2;
            two += 1;
        }
        while y % 5 == 0 {
            y /= 5;
            five += 1;
        }
        two = min(two, 18);
        five = min(five, 18);
        c.push((two, five));
        tfs[two][five] += 1i64;
    }
    let mut ans = 0i64;
    for &(two, five) in c.iter() {
        for i in (18 - two)..19 {
            for j in (18 - five)..19 {
                ans += tfs[i][j];
            }
        }
        if two >= 9 && five >= 9 {
            ans -= 1;
        }
    }
    println!("{}", ans >> 1);
}

const eps: f64 = 1e-10;
