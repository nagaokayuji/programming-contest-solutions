#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    a:i64,
    }

    let b = a;
    for k in 333..350 {
        let zz = b + k;
        let cxc = zz + b + k;
        println!("{}", zz);
    }
}

static MX: usize = 1010101;
static MOD: i64 = 1000000007;
static INF: i64 = std::i64::MAX >> 1;
