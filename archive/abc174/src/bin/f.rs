#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        q:usize,
        c:[Usize1;n],
        mut lr:[(Usize1,Usize1);q],
    };
    let mut bt = BIT::<SUM>::new(n + 111);
    let mut ql = vec![vec![]; n + 5];
    let mut qid = vec![vec![]; q + 5];
    for (i, &(l, r)) in lr.iter().enumerate() {
        ql[r].push(l);
        qid[r].push(i as usize);
    }

    let mut lst = vec![!0; n + 1];
    let mut anss = vec![0; q + 1];
    for i in 0..n {
        if lst[c[i]] != !0 {
            bt.add(lst[c[i]], &(-1));
        }
        bt.add(i, &1);
        lst[c[i]] = i;
        for j in 0usize..qid[i].len() as usize {
            anss[qid[i][j]] = bt.sum(i)
                - if ql[i][j] > 0 {
                    bt.sum(ql[i][j] - 1)
                } else {
                    0
                };
        }
    }

    for i in 0..q {
        println!("{}", anss[i]);
    }
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
            buf: vec![M::id(); n + 10000],
        }
    }
    pub fn sum(&self, i: usize) -> M::T {
        let mut i = i + 100;
        let mut s = M::id();
        while i > 0 {
            s = M::op(&s, &self.buf[i]);
            i -= 1 << i.trailing_zeros();
        }
        return s;
    }
    pub fn add(&mut self, i: usize, x: &M::T) {
        let mut i = i + 100;
        while i < self.buf.len() {
            let t = &mut self.buf[i];
            *t = M::op(&t, x);
            i += 1 << i.trailing_zeros();
        }
    }
}
