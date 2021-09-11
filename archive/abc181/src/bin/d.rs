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
    // mod8
    let mut hist = vec![0; 10];
    for &x in s.iter() {
        hist[charconv(x, '0')] += 1;
    }
    let possible = |y: i64| -> bool {
        let mut x = y;
        let mut his = hist.clone();
        // dbg!(&his);
        let mut ret = true;
        while x > 0 {
            let now = (x % 10) as usize;
            if his[now] > 0 {
                his[now] -= 1;
            } else {
                ret = false;
            }
            x /= 10;
        }
        return ret;
    };

    if s.len() == 1 {
        yn(hist[8] > 0);
        return;
    }
    if s.len() == 2 {
        for x in 10..100 {
            if x % 8 == 0 {
                if possible(x) {
                    yn(true);
                    return;
                }
            }
        }
        yn(false);
        return;
    }

    for x in 100..=1000 {
        // つくりたい
        if x % 8 == 0 {
            // dbg!((x, possible(x)));
            if possible(x) {
                yn(true);
                return;
            }
        }
    }
    yn(false);
}
pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
