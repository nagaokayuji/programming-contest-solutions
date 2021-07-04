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
        n:usize,k:i64,q:usize,
        mut a:[Usize1;q],
    }
    // k points
    let mut score = vec![k; n];
    let mut delete = 1i64 * q as i64;
    for &x in a.iter() {
        score[x] += 1;
    }
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
    score.iter().for_each(|&x| yn(x > delete));
}
