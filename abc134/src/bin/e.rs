#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[usize;n],
    }
    a.reverse();
    println!("{}", lis(&a));
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
#[derive(PartialEq, Eq, Clone, Debug)]
enum Inf<T> {
    Val(T),
    Inf,
}
impl<T: Ord> Inf<T> {
    #[allow(dead_code)]
    fn val(self) -> Option<T> {
        match self {
            Inf::Val(v) => Some(v),
            _ => None,
        }
    }
}
impl<T: PartialOrd> PartialOrd for Inf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Some(Ordering::Equal),
            (&Inf::Inf, &Inf::Val(_)) => Some(Ordering::Greater),
            (&Inf::Val(_), &Inf::Inf) => Some(Ordering::Less),
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.partial_cmp(b),
        }
    }
}
impl<T: Ord> Ord for Inf<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (&Inf::Inf, &Inf::Inf) => Ordering::Equal,
            (&Inf::Inf, &Inf::Val(_)) => Ordering::Greater,
            (&Inf::Val(_), &Inf::Inf) => Ordering::Less,
            (&Inf::Val(ref a), &Inf::Val(ref b)) => a.cmp(b),
        }
    }
}
/// Calculate length of Longest Increasing Subsequence. O(N log N)
pub fn lis<T: Ord>(seq: &[T]) -> usize {
    let mut dp: Vec<Inf<&T>> = vec![Inf::Inf; seq.len() + 1];
    for x in seq.iter() {
        let i = dp.upper_bound(&Inf::Val(x));
        dp[i] = Inf::Val(x);
    }
    dp.lower_bound(&Inf::Inf)
}
