#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    if s[2] == 'D' {
        print!("Doctor ");
    }
    if s[2] == 'M' {
        print!("Master ");
    }
    if s[2] == 'B' {
        print!("Bachelor ");
    }
    println!("{}{}", s[0], s[1]);
}

static MX: usize = 1010101;
static MOD: i64 = 1000000007;
static INF: i64 = std::i64::MAX >> 1;
