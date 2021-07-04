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
        n:usize,k:i64,
        mut a:[i64;n],
        mut f:[i64;n],
    }
    a.sort();
    f.sort();
    f.reverse();

    let mut ok = INF;
    let mut ng = -1i64;

    // dbg!(&a);
    // dbg!(&f);
    while ok - ng > 1 {
        let mid = (ok + ng) >> 1;
        let isOk = |x: i64| -> bool {
            let mut cnt = 0i64;
            for i in 0..n {
                if a[i] * f[i] > x {
                    let ml = x / f[i];
                    assert!(ml <= a[i]);
                    cnt += a[i] - ml;
                }
            }
            cnt <= k
        };
        if isOk(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
const INF: i64 = 1 << 60;

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
