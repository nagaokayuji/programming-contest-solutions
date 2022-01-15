#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
       mut ab:[(usize,usize);n],
    }
    ab.sort();
    let mut hp = BinaryHeap::new();
    let mut ind = 0;
    let mut ans = 0;
    for day in 1..=m {
        while ind < n && ab[ind].0 <= day {
            hp.push(ab[ind].1);
            ind += 1;
        }
        if let Some(x) = hp.pop() {
            ans += x;
        }
    }
    println!("{}", ans);
}
