use proconio::*;
use std::collections::*;

fn main() {
    input! {
        n:usize,m:usize,
        mut a:[i64;n],
        mut b:[i64;m],
    }
    a.sort();
    b.sort();

    let mut ans = 1e10 as i64;

    for &x in a.iter() {
        let bind = b.lower_bound(&x);
        if (0..m).contains(&bind) {
            ans = ans.min((x - b[bind]).abs());
        }
        if bind > 0 {
            ans = ans.min((x - b[bind - 1]).abs());
        }
    }
    println! {"{}",ans};
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
