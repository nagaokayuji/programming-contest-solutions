#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {n:usize,
        h:[usize;n],
    a:[u64;n],}
    let mut bt = BIT::<SUM>::new(n + 3);
    for i in 0..n {
        let p = bt.sum(h[i]);
        bt.add(h[i], &(p + a[i]));
    }
    println!("{}", bt.sum(n + 1));
}
pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}
pub struct SUM {}
impl Monoid for SUM {
    type T = u64;
    fn id() -> Self::T {
        0
    }
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        max(*a, *b)
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
