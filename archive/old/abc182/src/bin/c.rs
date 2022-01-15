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
    let k = s.len();

    let mut ans = 99;
    for bt in 0..1 << k {
        let mut nm = vec![];
        for i in 0..k {
            if bt >> i & 1 == 1 {
                // のこる
                nm.push(s[i]);
            }
        }
        if chari(nm) % 3 == 0 {
            if (bt as i64).count_ones() == 0 as u32 {
                continue;
            }
            ans = min(ans, k - (bt as i64).count_ones() as usize);
        }
    }
    if ans == 99 {
        println!("{}", -1);
        return;
    }
    println!("{}", ans);
}

fn chari(s: Vec<char>) -> i64 {
    let mut ret = 0i64;
    for &x in s.iter() {
        ret *= 10;
        ret += charconv(x, '0') as i64;
    }
    ret
}
pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
