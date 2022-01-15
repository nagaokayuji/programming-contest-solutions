#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,m:usize,
       mut  h:[i64;n],
        mut w:[i64;m],
    }
    if n == 1 {
        let target = h[0];
        let mut ans = INF;
        for &x in w.iter() {
            ans = min(ans, (x - target).abs());
        }
        println!("{}", ans);
        return;
    }
    h.sort();
    w.sort();
    let mut eos = vec![0; n / 2 + 2];
    let mut oes = vec![0; n / 2 + 2];
    for i in 0..n / 2 {
        eos[i + 1] = eos[i] + (h[i * 2] - h[i * 2 + 1]).abs();
        oes[i + 1] = oes[i] + (h[i * 2 + 1] - h[i * 2 + 2]).abs();
    }
    // dbg!((&eos, &oes));

    let mut ans = INF;
    for i in 0..=(n / 2) {
        // dbg!(i);
        let target = h[i * 2];
        let mut tc1 = w.lower_bound(&target);
        if tc1 >= m {
            tc1 -= 1;
        }
        let mut tc2 = tc1 - if tc1 > 0 { 1 } else { 0 };

        let k1 = w[tc1];
        let k2 = w[tc2];

        let ad = min((target - k1).abs(), (target - k2).abs());
        // dbg!((ad, ad + eos[i] + oes[n / 2] - oes[i]));
        ans = min(ans, ad + eos[i] + oes[n / 2] - oes[i]);
    }
    println!("{}", ans);

    // dbg!(&dp[0]);
    // dbg!(&dp[n - 1]);
    // dp[2] =
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
