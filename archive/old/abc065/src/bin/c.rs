#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:i64,m:i64,
    }
    if (n - m).abs() > 1 {
        println!("{}", 0);
        return;
    }

    // n! * m!
    let mut fact = vec![1i64; 123456];
    for i in 2..123456 {
        fact[i] = fact[i - 1] * i as i64;
        fact[i] %= MOD;
    }

    println!(
        "{}",
        fact[n as usize] * fact[m as usize] * if n == m { 2 } else { 1 } % MOD
    );
}
const MOD: i64 = 1000000007;
