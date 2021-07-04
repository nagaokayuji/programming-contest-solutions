#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,d:usize,
        x:[[i64;d];n],
    }
    let mut nijos = BTreeSet::new();
    for i in 0..150 {
        nijos.insert(i * i);
    }
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut root = 0;
            for ten in 0..d {
                root += (x[i][ten] - x[j][ten]) * (x[i][ten] - x[j][ten]);
            }
            if nijos.contains(&root) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
