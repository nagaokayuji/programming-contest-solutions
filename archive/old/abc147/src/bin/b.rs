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
    let mut c = 0;
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            c += 1;
        }
    }
    println!("{}", c);
}
