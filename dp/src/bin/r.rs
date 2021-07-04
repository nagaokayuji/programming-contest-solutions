#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:usize,k:i64,
    a:[ [i64;n];n],
     }
    let mut mata = Matrix::new(a.clone());
    let matp = mata.pow(k as usize, MOD);
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            ans = (ans + matp[i][j]) % MOD;
        }
    }
    println!("{}", ans);
}
const MOD: i64 = 1000000007;

struct Matrix {
    m: Vec<Vec<i64>>,
}
impl Matrix {
    fn new(v: Vec<Vec<i64>>) -> Matrix {
        Matrix { m: v }
    }
    fn set(&mut self, v: Vec<Vec<i64>>) {
        self.m = v.clone();
    }
    fn dot(&self, m: &Matrix, md: i64) -> Vec<Vec<i64>> {
        let a = self.m.len();
        let b = self.m[0].len();
        let c = m.m[0].len();
        let mut ret = vec![vec![0; c]; a];
        for i in 0..a {
            for j in 0..c {
                for k in 0..b {
                    ret[i][j] += (self.m[i][k] % md) * (m.m[k][j] % md) % md;
                }
            }
        }
        ret
    }
    fn pow(&mut self, k: usize, md: i64) -> Vec<Vec<i64>> {
        let n = self.m.len();
        let mut y = vec![vec![0; n]; n];
        for i in 0..n {
            y[i][i] = 1;
        }
        let mut z = self.m.clone();
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                y = Matrix::new(y).dot(&Matrix::new(z.clone()), md);
            }
            z = Matrix::new(z.clone()).dot(&Matrix::new(z.clone()), md);
            k >>= 1;
        }
        y
    }
}
