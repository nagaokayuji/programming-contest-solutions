#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

fn main() {
    input! {t:usize}
    for _ in 0..t {
        solve();
    }
}
fn solve() {
    input! {n:usize,
    klr:[(usize,i64,i64);n]}
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut same_sum = 0;
    for &(k, l, r) in klr.iter() {
        if l > r {
            left.push((k, l, r));
        } else if r > l {
            right.push((n - k, r, l));
        } else {
            same_sum += l;
        }
    }
    println!("{}", same_sum + calc(&mut left, n) + calc(&mut right, n));
}

fn calc(camels: &mut Vec<(usize, i64, i64)>, N: usize) -> i64 {
    camels.sort_by_key(|&x| -x.1 + x.2);
    let n = camels.len();
    let mut ret = 0;
    let mut bt = BIT::<MAX>::new(N + 2);
    let base = 1;
    for i in 0..=n {
        // seg.update(i, i as i64);
        bt.add(i + base, &((base + i) as i64));
    }
    for &(k, l, r) in camels.iter() {
        // // k 未満の最小のインデックスで未使用のものを探す
        let ind = bt.sum(k + base);
        if ind < 0 {
            ret += r;
        } else {
            ret += l;
            // seg.update(ind as usize, -1);
            bt.add(base + ind as usize, &(-1));
        }
    }
    return ret;
}
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}
pub struct MAX;
impl Monoid for MAX {
    type T = i64;
    fn id() -> Self::T {
        -1
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        max(*a, *b)
    }
}
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
            i &= i - 1;
        }
        s
    }
    pub fn add(&mut self, i: usize, x: &M::T) {
        let mut i = i as i64;
        while i < self.buf.len() as i64 {
            let t = &mut self.buf[i as usize];
            *t = M::op(&t, x);
            i += i & -i;
        }
    }
}
