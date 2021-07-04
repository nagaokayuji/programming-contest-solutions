#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {n:usize,
    a:[i64;n],}

    let mut ds = vec![0; n + 1];
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    for i in 0..n {
        ds[i + 1] = ds[i] + a[i];
    }
    let mut g = G {
        n: n,
        a: a,
        ds: ds,
        dp: dp,
    };
    println!("{}", g.f(1, n));
}
struct G {
    n: usize,
    a: Vec<i64>,
    ds: Vec<i64>,
    dp: Vec<Vec<i64>>,
}

impl G {
    fn f(&mut self, l: usize, r: usize) -> i64 {
        if self.dp[l][r] != -INF {
            return self.dp[l][r];
        }
        if l == r {
            self.dp[l][r] = 0;
            return 0;
        }
        // dbg!(l, r);
        let mut ret = INF;
        for m in l..r {
            ret = min(ret, self.f(l, m) + self.f(m + 1, r));
        }
        self.dp[l][r] = ret + self.ds[r] - self.ds[l - 1];
        return self.dp[l][r];
    }
}

const INF: i64 = 1 << 60;
