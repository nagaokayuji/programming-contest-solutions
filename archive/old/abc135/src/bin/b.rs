#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
       mut p:[Usize1;n],
    }

    let mut cnt = 0;
    for i in 0..n {
        if i != p[i] {
            cnt += 1;
        }
    }
    if cnt == 0 || cnt == 2 {
        println!("YES");
        return;
    }
    println!("NO");
}
