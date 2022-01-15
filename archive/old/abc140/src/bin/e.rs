#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        a:[usize;n],
    }
    let mut inv = vec![0; n + 2];
    for i in 0..n {
        inv[a[i]] = i + 1;
    }
    let mut l = (0..n + 3).map(|i| max(i, 1) - 1).collect::<Vec<_>>();
    let mut r = (0..n + 3).map(|i| min(i, n) + 1).collect::<Vec<_>>();
    let mut ans = 0;
    // i<n
    for i in 1..n {
        let pos = inv[i];
        let a = l[pos];
        let b = l[a];
        let c = r[pos];
        let d = r[c];
        ans += i * ((a - b) * (c - pos) + (pos - a) * (d - c));
        l[c] = a;
        r[a] = c;
    }
    println!("{}", ans);
}
