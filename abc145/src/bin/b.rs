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
        n:usize,
        s:Chars,
    }
    if n % 2 != 0 {
        yn(false);
        return;
    }
    for i in 0..n / 2 {
        if s[i] != s[i + n / 2] {
            yn(false);
            return;
        }
    }
    yn(true);
}

fn yn(ans: bool) {
    println!("{}", if ans { "Yes" } else { "No" });
}
