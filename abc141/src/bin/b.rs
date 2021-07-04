#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        s:Chars,
    }
    let ans = s
        .iter()
        .enumerate()
        .filter(|&x| x.1 == &'R' || x.1 == &'L')
        .any(|x| (x.0 % 2 == 0 && x.1 == &'L') || (x.0 % 2 == 1 && x.1 == &'R'));
    yn(!ans);
}
fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}
