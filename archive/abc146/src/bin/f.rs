#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
        mut s:Chars,
    }
    s.reverse();
    let mut left = 0;
    let mut anss = vec![];
    let mut ng = false;
    while left < n {
        let mut right = left + m;
        let mut found = false;
        for ms in (1..=m).rev() {
            let target = left + ms;
            if target <= n && s[target] == '0' {
                left = target;
                anss.push(ms);
                found = true;
                break;
            }
        }
        if !found {
            ng = true;
            break;
        }
    }
    if ng {
        println!("{}", -1);
    } else {
        for &x in anss.iter().rev() {
            println!("{}", x);
        }
    }
}
