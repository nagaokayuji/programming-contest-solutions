#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut d:[(usize,usize);n],
    }
    let mut ok = false;
    for i in 2..n {
        if d[i - 2].0 == d[i - 2].1 && d[i - 1].0 == d[i - 1].1 && d[i].0 == d[i].1 {
            ok = true;
        }
    }
    yn(ok);
}

pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
