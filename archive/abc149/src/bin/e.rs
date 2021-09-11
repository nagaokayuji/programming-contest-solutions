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
        n:usize,m:usize,
        mut a:[i64;n],
    }
    a.sort();
    let mut cum = vec![0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    let mut ok = 1;
    let mut ng = 555555;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        let mut cnt = 0;
        let mut j = 0;
        for i in (0..n).rev() {
            while j < n && a[i] + a[j] < mid {
                j += 1;
            }
            cnt += n - j;
        }
        if cnt >= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let mut cnt = 0;
    let mut sum = 0i64;
    for i in 0..n {
        let c = a.lower_bound(&(ok - a[i]));
        cnt += n - c;
        sum += a[i] * (n as i64 - c as i64) + cum[n] - cum[c];
    }
    sum -= (cnt as i64 - m as i64) * ok;
    println!("{}", sum);
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
