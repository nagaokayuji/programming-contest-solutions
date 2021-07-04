#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        l:i64,a:i64,b:i64,MOD:i64,
    }
    // 初項a, 公差b, 長さl, mod m

    // let mut v = Matrix::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    // let mut w = Matrix::new(vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    // let ans = v.dot(&w, 99999);
    // dbg!(&ans);
    // dbg!(&v.pow(4));
    let mut x = 1;
    let mut ans = 0;
    let mut l = l;
    let mut a = a;
    while l > 0 {
        x *= 10;
        if a >= x {
            continue;
        }
        let mut l2 = min(l, (x - a - 1) / b + 1);
        let mut mat = Matrix::new(vec![vec![1, 0, b], vec![1, x, 0], vec![0, 0, 1]]);
        let mut aa = mat.pow(l2 as usize, MOD);
        ans = ((a % MOD) * aa[1][0] % MOD) + (ans * aa[1][1] % MOD) + aa[1][2] % MOD;
        ans %= MOD;
        ans = (ans + MOD) % MOD;
        l -= l2;
        a += b * l2;
        // a = (a + MOD) % MOD;
    }
    println!("{}", ans);
}

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
        // (a,b) -> (b,c)
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
