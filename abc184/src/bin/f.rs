#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,t:i64,
        a:[i64;n],
    }
    let mut zen = a.iter().take(n / 2).collect::<Vec<_>>();
    let mut go = a.iter().skip(n / 2).collect::<Vec<_>>();

    let n1 = zen.len();
    let n2 = go.len();
    let mut mg1 = vec![];
    for bt in 0..1 << n1 {
        let mut tmp = 0i64;
        for i in 0..n1 {
            if bt >> i & 1 == 1 {
                tmp += zen[i];
            }
        }
        mg1.push(tmp);
    }
    let mut mg2 = vec![];
    for bt in 0..1 << n2 {
        let mut tmp = 0i64;
        for i in 0..n2 {
            if bt >> i & 1 == 1 {
                tmp += go[i];
            }
        }
        mg2.push(-tmp);
    }
    mg1.sort();
    mg2.sort();
    let mut ans = 0i64;
    for &x in mg1.iter() {
        if x > t {
            break;
        }
        let target = x - t;
        let k = mg2.lower_bound(&target);
        ans = max(ans, x - mg2[k]);
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
