#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        t:usize,
    }
    for _ in 0..t {
        solve();
    }
}
fn solve() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    if n % 2 == 1 {
        yn(false);
        return;
    }
    let mut mp = BTreeMap::new();
    for &x in a.iter() {
        *mp.entry(x).or_insert(0) += 1;
    }
    yn(!mp.iter().all(|(k, v)| v % 2 == 0));
}
pub fn yn(ans: bool) {
    if ans {
        println!("First");
    } else {
        println!("Second");
    }
}
