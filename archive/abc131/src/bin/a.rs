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
    let mut ok = false;
    if s[0] == s[1] || s[1] == s[2] || s[2] == s[3] {
        ok = true;
    }
    println!("{}", if ok { "Bad" } else { "Good" });
}
