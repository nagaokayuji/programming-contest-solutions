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
        mut a:i64,mut b:i64,mut k:i64
    }
    let tk = min(a, k);
    a -= tk;
    k -= tk;
    let tt = min(b, k);
    b -= tt;
    k -= tt;
    println!("{} {}", a, b);
}
