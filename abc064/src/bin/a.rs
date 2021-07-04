#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        r:i64,g:i64,b:i64
    }
    yn((r * 100 + g * 10 + b) % 4 == 0);
}
pub fn yn(ans: bool) {
    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
