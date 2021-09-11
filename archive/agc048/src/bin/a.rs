#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        t:usize,
        s:[Chars;t],
    }
    for t in s.iter() {
        solve(&t);
    }
}
fn solve(s: &Vec<char>) {
    let n = s.len();
    if n == 1 {
        if s[0] == 'a' {
            println!("{}", -1);
        } else {
            println!("{}", 0);
        }
        return;
    }
    let mut ac = s.iter().filter(|&&x| x == 'a').count();
    if ac == n {
        println!("{}", -1);
        return;
    }
    let mut atcoder = "atcoder".chars().collect::<Vec<char>>();
    if &atcoder < s {
        println!("{}", 0);
        return;
    }
    let mut firstno = 0;
    if s[0] != 'a' {
        println!("{}", 0);
        return;
    }

    for i in 0..n {
        if s[i] != 'a' {
            firstno = i;
            break;
        }
    }
    assert!(firstno > 0);
    if firstno <= 1 {
        println!("{}", firstno);
        return;
    } else {
        if s[firstno] > 't' {
            println!("{}", firstno - 1);
            return;
        } else {
            println!("{}", firstno);
            return;
        }
    }
}
