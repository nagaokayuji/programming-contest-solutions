#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        mut l:i128,
    }
    if l == 12 {
        println!("{}", 1);
        return;
    }
    let n = l - 1;
    let m = 11;
    // nCm
    // let npm = println!("{}", md.comb(l as i128 - 1, 11));
    let mut npm = 1i128;
    for i in (n - 11..=n) {
        npm *= i;
    }
    let mut elevens = 1i128;
    for i in (1..=11) {
        elevens *= i;
    }
    println!("{}", npm / elevens);
}
const MX: usize = 1010101;
const MOD: i128 = 1i128 << 63;
const INF: i128 = 1 << 60;
