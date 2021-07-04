#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,
        vu:[(Usize1,Usize1);n-1],
    }
    let mut ans = 0;
    for i in 0..n {
        ans += (i + 1) * (n - i);
    }
    for (x, y) in vu.iter() {
        let mut x = x;
        let mut y = y;
        if x > y {
            swap(&mut x, &mut y);
        }
        ans -= (x + 1) * (n - y);
    }
    println!("{}", ans);
}
