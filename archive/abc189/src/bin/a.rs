#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        c:Chars,
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Won" } else { "Lost" });
    }
    yn(c[0] == c[1] && c[1] == c[2]);
}
