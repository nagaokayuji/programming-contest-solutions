#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        s:Chars,
    }
    let mut nm = 0i64;
    for &x in s.iter() {
        if x == '(' {
            nm += 1;
        } else {
            nm -= 1;
        }
        if nm < 0 {
            print!("(");
            nm += 1;
        }
    }
    for &x in s.iter() {
        print!("{}", x);
    }
    if nm > 0 {
        for _ in 0..nm.abs() {
            print!(")");
        }
    }
    println!();
}
