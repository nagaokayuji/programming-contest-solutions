#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,m:i64,
        a:[i64;n],
    }
    let asum: i64 = a.iter().sum();
    if asum > m {
        println!("{}", 0);
        return;
    }
    if asum == m {
        println!("{}", 1);
        return;
    }
    let left = m - asum;
    assert!(left > 0);

    // n^2 くらい
}

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
struct ModComb {
    fact: Vec<i64>,
    fact_inv: Vec<i64>,
    n: usize,
    p: i64,
}
impl ModComb {
    /// 前計算
    fn initialize(&mut self) {
        let n = self.n;
        self.fact[0] = 1;
        for i in 1..n {
            self.fact[i] = (self.fact[i - 1] * i as i64) % self.p;
        }
        self.fact_inv[n - 1] = mod_pow(self.fact[n - 1], self.p - 2, self.p);
        for i in (0..n - 1).rev() {
            self.fact_inv[i] = (self.fact_inv[i + 1] * (i + 1) as i64) % self.p;
        }
    }
    fn new(max_n: usize, p: i64) -> ModComb {
        let mut ft = ModComb {
            fact: vec![0; max_n + 1],
            fact_inv: vec![0; max_n + 1],
            n: max_n + 1,
            p: p,
        };
        ModComb::initialize(&mut ft);
        ft
    }
    /// factorial
    fn fact(&self, n: usize) -> i64 {
        self.fact[n]
    }
    /// combination
    fn comb(&self, n: i64, k: i64) -> i64 {
        if n < k {
            return 0;
        }
        (self.perm(n, k) * self.fact_inv[k as usize]) % self.p
    }
    /// permutation
    fn perm(&self, n: i64, k: i64) -> i64 {
        if n < k {
            return 0;
        }
        self.fact[n as usize] * self.fact_inv[(n - k) as usize] % self.p
    }
    /// スターリング
    /// 区別できるn個のものを区別できないkグループに分類する場合の数
    /// (各グループ1個以上)
    fn stirling(&self, n: i64, k: i64) -> i64 {
        if n < k {
            return 0;
        }
        let mut res = 0;
        for i in 0..k + 1 {
            let v = self.comb(k, i) * mod_pow(i, n, self.p) % self.p;
            if (k - i) % 2 == 1 {
                res = (res + self.p - v) % self.p;
            } else {
                res = (res + v) % self.p;
            }
        }
        return res * self.fact_inv[k as usize] % self.p;
    }
}