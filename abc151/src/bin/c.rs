#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        n:usize,m:usize,
        ps:[(Usize1,Chars);m],
    }
    let mut ac = vec![false; n];
    let mut penalty = vec![0; n];
    for (p, s) in &ps {
        if s[0] == 'A' {
            ac[*p] = true;
        } else {
            if !ac[*p] {
                penalty[*p] += 1;
            }
        }
    }
    let mut acc = 0;
    let mut penalties = 0;
    for i in 0..n {
        if ac[i] {
            acc += 1;
            penalties += penalty[i];
        }
    }
    println!("{} {}", acc, penalties);
}
