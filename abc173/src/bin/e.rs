#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,k:usize,
        mut a:[i64;n],
    }
    let mut neg = Vec::new();
    let mut pos = Vec::new();
    let mut zeros = 0;
    a.sort_by_key(|x| x.abs());
    for &x in a.iter() {
        if x < 0 {
            neg.push(x);
        } else if x >= 0 {
            pos.push(x);
        }
        if x == 0 {
            zeros += 1;
        }
    }
    if k > n - zeros {
        println!("{}", 0);
        return;
    }

    if k == n {
        let mut ret = 1;
        for x in a.iter() {
            ret *= x;
            ret %= MOD;
            ret = (ret + MOD) % MOD;
        }
        println!("{}", ret);
        return;
    }
    if a.iter().all(|&x| x < 0) && k % 2 == 1 {
        let mut ret = 1i64;
        for x in a.iter().take(k) {
            ret *= x;
            ret %= MOD;
            ret = (ret + MOD) % MOD;
        }
        println!("{}", ret);
        return;
    }
    let mut ret = 1;
    let mut rest = k;
    while rest > 0 {
        if pos.len() >= 1
            && (rest == 1
                || neg.len() < 2
                || pos[pos.len() - 1] * *pos.get(pos.len() - 2).unwrap_or(&&1)
                    >= neg[neg.len() - 1] * neg[neg.len() - 2])
        {
            ret *= pos.pop().unwrap();
            ret %= MOD;
            rest -= 1;
        } else {
            ret *= neg.pop().unwrap();
            ret %= MOD;
            ret *= neg.pop().unwrap();
            ret %= MOD;
            rest -= 2;
        }
    }

    println!("{}", ret);
}

const MOD: i64 = 1000000007;
const MMOD: i64 = MOD * MOD;
const INF: i64 = 1 << 55;
