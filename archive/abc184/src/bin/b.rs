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
        n:usize,
        x:i64,
        s:Chars,
    }
    let mut ans = x;
    for &x in s.iter() {
        if x == 'o' {
            ans += 1;
        } else {
            if ans > 0 {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
