#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:Chars,
    }
    pub fn yn(ans: bool) {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    let v = n.iter().fold(0, |sum, &x| sum + charconv(x, '0'));
    yn(v % 9 == 0);
}
pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
