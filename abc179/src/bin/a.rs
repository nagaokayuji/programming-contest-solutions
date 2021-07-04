#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        s:Chars,
    }
    let n = s.len();
    for &x in &s {
        print!("{}", x);
    }
    if s[n - 1] == 's' {
        println!("{}", "es");
        return;
    }
    println!("{}", "s");
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
