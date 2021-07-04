#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        a:[i64;n],
    }
    // min
    let mut s = BTreeSet::new();
    let mut f = 0;
    for &x in a.iter() {
        if x >= 3200 {
            f += 1;
        } else {
            s.insert(x / 400);
        }
    }
    println!("{}", if s.len() == 0 { 1 } else { s.len() });
    println!("{}", s.len() + f);
}
