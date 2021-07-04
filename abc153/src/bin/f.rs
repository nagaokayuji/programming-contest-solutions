#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,d:i64,a:i64,
        mut xh: [(i64,i64);n],
    }
    xh.sort();
    let x: Vec<i64> = xh.iter().map(|&x| x.0).collect::<Vec<_>>();
    let mut h: Vec<i64> = xh.iter().map(|&x| x.1).collect::<Vec<_>>();
    let mut ans = 0i64;
    let mut r = 0usize;
    let mut t = vec![0; n + 2];
    for left in 0..n {
        if left > 0 {
            t[left] += t[left - 1];
        }
        let mut now = h[left] - t[left];
        if now <= 0 {
            continue;
        }
        let cnt = (now + a - 1) / a;
        ans += cnt;
        let minus = cnt * a;
        r = x.upper_bound(&(x[left] + 2 * d));
        t[left] += minus;
        t[r] -= minus;
    }
    println!("{}", ans);
}

pub trait BinarySearch<T> {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize;
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while (ok as i32 - ng as i32).abs() > 1 {
            let mid = (ok + ng) / 2;
            if f(&self[mid as usize]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn lower_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y >= x)
    }
    fn upper_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y > x)
    }
}
