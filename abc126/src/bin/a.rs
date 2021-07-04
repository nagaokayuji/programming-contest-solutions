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
        k:Usize1,
        s:Chars,
    }

    s.iter()
        .enumerate()
        .map(|(i, &x)| {
            if i == k {
                x.to_lowercase().to_string()
            } else {
                x.to_uppercase().to_string()
            }
        })
        .for_each(|y| print!("{}", y));
}
