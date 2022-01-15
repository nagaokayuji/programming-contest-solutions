#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:usize,
    s:[String;n],
     }
    let mut mp = BTreeMap::new();
    for strn in s.iter() {
        *mp.entry(strn.as_str()).or_insert(0i64) += 1i64;
    }
    for c in ["AC", "WA", "TLE", "RE"].iter() {
        println!("{} x {}", c, *mp.entry(c).or_insert(0));
    }
}
