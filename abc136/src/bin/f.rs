#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
       mut p:[(i64,i64);n],
    }

    p.sort();
    for (i, v) in p.iter_mut().enumerate() {
        v.0 = (i + 1) as i64;
        swap(&mut v.0, &mut v.1);
    }
    p.sort();
    let mut l = BIT::<SUM>::new(n);
    let mut r = BIT::<SUM>::new(n);
    for i in 1..=n {
        r.add(i, &1);
    }
    let n = n as i64;
    let mut ans = n as i64 * mod_pow(2, n - 1, MOD);
    for (i, p) in p.iter().enumerate() {
        let i = i as i64;
        let y = p.1 as usize;
        r.add(y, &(-1i64));
        let a = l.sum(y);
        let b = (i - a as i64);
        let c = r.sum(y);
        let d = (n - 1 - i) - c;
        ans += mod_pow(2, n - 1, MOD)
            + mod_pow(2, a, MOD)
            + mod_pow(2, b, MOD)
            + mod_pow(2, c, MOD)
            + mod_pow(2, d, MOD)
            - mod_pow(2, a + b, MOD)
            - mod_pow(2, a + c, MOD)
            - mod_pow(2, b + d, MOD)
            - mod_pow(2, c + d, MOD)
            - 1;
        ans = (ans + MOD * 111) % MOD;
        l.add(y, &1i64);
    }
    println!("{}", ans);
}

const MOD: i64 = 998244353;
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
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}
pub struct SUM {}
impl Monoid for SUM {
    type T = i64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}
/// Generic Binary Indexed Tree
pub struct BIT<M: Monoid> {
    buf: Vec<M::T>,
}
impl<M: Monoid> BIT<M> {
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![M::id(); n + 1],
        }
    }
    pub fn sum(&self, i: usize) -> M::T {
        let mut i = i;
        let mut s = M::id();
        while i > 0 {
            s = M::op(&s, &self.buf[i]);
            i -= 1 << i.trailing_zeros();
        }
        s
    }
    pub fn add(&mut self, i: usize, x: &M::T) {
        let mut i = i;
        while i < self.buf.len() {
            let t = &mut self.buf[i];
            *t = M::op(&t, x);
            i += 1 << i.trailing_zeros();
        }
    }
}
