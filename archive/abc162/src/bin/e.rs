#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

const MX: usize = 1010101;
#[fastout]
fn main() {
    input! {
        n:i64,k:i64,
    }
    let mut ans = 0;
    let mut cnts = vec![0; MX];
    for gcds in (1..=k).rev() {
        let mut l = k / gcds;

        let mut al = mod_pow(l, n, MOD);
        for mul in 2..=l {
            al -= cnts[(mul * gcds) as usize];
            al = (al + MOD) % MOD;
        }
        cnts[gcds as usize] = al;
        ans += al * gcds;
        ans %= MOD;
    }
    println!("{}", ans);
}
const MOD: i64 = 1000000007;
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
