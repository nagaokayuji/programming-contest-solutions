#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        a:[i64;4],
    }
    pub fn yn(ans: bool) {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    let mut koho = vec![];
    for &x in a.iter() {
        koho.push(x - 1);
        koho.push(x);
        koho.push(x + 1);
    }
    let mut ok = false;
    for &x in koho.iter() {
        if a[0] <= x && x <= a[1] && a[2] <= x && x <= a[3] {
            ok = true;
        }
    }
    yn(ok);
}
