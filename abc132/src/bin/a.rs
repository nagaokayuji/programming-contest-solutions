#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        s:Bytes,
    }
    let mut mp = BTreeMap::new();
    for &x in s.iter() {
        *mp.entry(x).or_insert(0) += 1;
    }
    for (key, value) in mp {
        if value != 2 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
