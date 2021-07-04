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
        n:usize,k:usize,
        mut s:Chars,
    }
    let rl = run_length_encoding(&s);
    let n = rl.len();
    // lllrrrllrl
    // 5 になる
    //
    let mut score = rl.iter().fold(0, |sum, x| sum + x.1 - 1);
    // k -> 2増えそう
    println!("{}", min(score + 2 * k, s.len() - 1));
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
