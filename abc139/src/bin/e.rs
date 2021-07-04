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
        a:[[Usize1;n-1];n],
    }

    let mut id = vec![vec![0; n]; n];
    let mut v = 0usize;
    for i in 0..n {
        for j in 0..n {
            if i < j {
                id[i][j] = v;
                v += 1;
            }
        }
    }
    let getId = |i: usize, j: usize| {
        if i > j {
            id[j][i]
        } else {
            id[i][j]
        }
    };
    let mut a = a.clone();
    let mut to = vec![vec![]; n * n];
    for i in 0..n {
        for j in 0..n - 1 {
            a[i][j] = getId(i, a[i][j]);
        }
        for j in 0..n - 2 {
            to[a[i][j + 1]].push(a[i][j]);
        }
    }
    // to
    let mut g = G {
        to: to,
        dp: vec![0; v],
        visited: vec![false; v],
        calculated: vec![false; v],
    };
    for i in 0..v {
        if g.dfs(i) == !0 {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", g.dp.iter().max().unwrap());
}
const MX: usize = 4101;
struct G {
    to: Vec<Vec<usize>>,
    dp: Vec<usize>, // max length from v
    visited: Vec<bool>,
    calculated: Vec<bool>,
}
impl G {
    fn dfs(&mut self, v: usize) -> usize {
        if self.visited[v] {
            if !self.calculated[v] {
                return !0;
            }
            return self.dp[v];
        }
        self.visited[v] = true;
        self.dp[v] = 1;
        for &nx in self.to[v].clone().iter() {
            if self.dfs(nx) == !0 {
                return !0;
            }
            self.dp[v] = max(self.dp[v], self.dfs(nx) + 1);
        }
        self.calculated[v] = true;
        self.dp[v]
    }
}
