#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
#[fastout]
fn main() {
    input! {
        n:usize,p:usize,
        s:Chars,
    }
    let mut s = s;
    if p == 2 || p == 5 {
        let mut ans = 0i64;
        for (i, &c) in s.iter().enumerate() {
            if char_conv(c, '0') % p == 0 {
                ans += (i + 1) as i64;
            }
        }
        println!("{}", ans);
        return;
    }
    s.reverse();
    let mut tens = 1usize;
    let mut nm = 0;
    let mut ls = vec![0; p + 3];
    ls[0] = 1;
    for (i, &c) in s.iter().enumerate() {
        let now = char_conv(c, '0');
        nm = (now * tens + nm) % p;
        tens = (tens * 10) % p;
        ls[nm] += 1;
    }
    let ans = ls.iter().fold(0i64, |sum, &cnt| sum + cnt * (cnt - 1) / 2);
    println!("{}", ans);
}
pub fn char_conv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
