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
        mut c:[i64;n],
    }
    c.sort();
    let mut ans = 0i64;
    let p = mod_pow(4, (n - 1) as i64, MOD);
    for i in 0..n {
        ans = (ans + (c[i] * (2 + (n - i - 1) as i64) % MOD) * p) % MOD;
    }
    println!("{}", ans);
}
const MOD: i64 = 1000000007;
const MX: usize = 1010101;
/// x ^ n % m
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}
