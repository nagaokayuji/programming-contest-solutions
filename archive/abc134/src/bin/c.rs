#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
      mut  a:[i64;n],
    }
    let mut b = vec![];
    for (i, &x) in a.iter().enumerate() {
        b.push((x, i));
    }
    b.sort();
    b.reverse();
    let mxi = b[0].1;

    for i in 0..n {
        if i == mxi {
            println!("{}", b[1].0);
        } else {
            println!("{}", b[0].0);
        }
    }
}
