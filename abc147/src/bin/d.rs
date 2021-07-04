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
        a:[i64;n],
    }
    let mut ans = 0i64;
    (0..=60i64).for_each(|i| {
        let one = a.iter().filter(|&&x| x >> i & 1i64 == 1).count() as i64;
        ans += ((1i64 << i) % MOD) * ((one * (n as i64 - one)) % MOD) % MOD;
        ans %= MOD;
    });
    println!("{}", ans);
}
const MOD: i64 = 1000000007;
