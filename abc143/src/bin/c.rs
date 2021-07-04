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
        n:usize,
        s:Chars,
    }
    println!("{}", run_length_encoding(&s).len());
}

fn run_length_encoding<T: Copy + Eq>(a: &Vec<T>) -> Vec<(T, usize)> {
    let n = a.len();
    let mut l = 0;
    let mut r = 0;
    let mut ret = vec![];
    while l < n {
        while r < n && a[l] == a[r] {
            r += 1;
        }
        ret.push((a[l], r - l));
        l = r;
    }
    ret
}
