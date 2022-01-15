#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        xy:[(Usize1,Usize1);n-1],
    }
    let mut v = vec![vec![]; n];
    for &(x, y) in xy.iter() {
        v[x].push(y);
        v[y].push(x);
    }
    // // root = 0
    // //
    // let mut dp = vec![vec![0; 2]; n + 1];//dp[i][j] := i
    // let mut q = VecDeque::new();
    let mut g = G::new(&v);
    g.dfs(0, !0);
    println!("{}", (g.dp[0][0] + g.dp[0][1]) % MOD);
}
struct G {
    n: usize,
    v: Vec<Vec<usize>>,
    dp: Vec<Vec<i64>>,
}
impl G {
    fn new(v: &Vec<Vec<usize>>) -> G {
        G {
            n: v.len(),
            v: v.clone(),
            dp: vec![vec![0i64; 2]; v.len()],
        }
    }
    fn dfs(&mut self, now: usize, prev: usize) {
        self.dp[now][0] = 1;
        self.dp[now][1] = 1;

        for &nx in self.v[now].clone().iter() {
            if nx != prev {
                self.dfs(nx, now);
                self.dp[now][0] = self.dp[now][0] * (self.dp[nx][0] + self.dp[nx][1]) % MOD;
                self.dp[now][1] = self.dp[now][1] * self.dp[nx][0] % MOD;
            }
        }
    }
}
const MOD: i64 = 1000000007;
