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
        n:usize,
        s:Chars,
    }

    let fox = vec!['f', 'o', 'x'];
    let mut l = 0;
    let mut r = 0;
    let mut ss = vec![];
    let mut omg = 0;
    while l < n {
        if !fox.contains(&s[l]) {
            omg += 1;
            l += 1;
            continue;
        }
        r = l;
        let mut tmp = vec![];
        while r < n && fox.contains(&s[r]) {
            tmp.push(s[r]);
            r += 1;
        }
        ss.push(tmp);
        l = r;
    }
    // dbg!(&ss);
    for x in ss.iter() {
        omg += solve(&x);
    }
    println!("{}", omg);
}
fn solve(s: &Vec<char>) -> usize {
    let mut omg = 0;
    let n = s.len();
    let mut s = s.clone();
    let mut fs = vec![];
    let mut os = vec![];
    let mut xs = vec![];
    for i in 0..n {
        if s[i] == 'f' {
            fs.push(i);
        }
        if s[i] == 'o' {
            os.push(i);
        }
        if s[i] == 'x' {
            xs.push(i);
        }
    }
    let mut q = vec![];
    let mut ok = 0;
    for &x in s.iter().rev() {
        q.push(x);
    }
    let mut i = 0;
    let mut fc = 0;
    let mut foc = 0;
    let mut q = vec![];
    while i < n {
        if i + 2 < n && s[i] == 'f' && s[i + 1] == 'o' && s[i + 2] == 'x' {
            ok += 3;
            i += 3;
            continue;
        }
        if s[i] == 'x' {
            if q.len() >= 2 {
                let k = q.len();
                if q[k - 2] == 'f' && q[k - 1] == 'o' {
                    q.pop();
                    q.pop();
                    ok += 3;
                    i += 1;
                    continue;
                }
            }
        }
        q.push(s[i]);

        i += 1;
    }
    n - ok
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
