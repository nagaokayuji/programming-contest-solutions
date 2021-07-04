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
        a:[Usize1;n],
    }
    let mut ai = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<Vec<_>>();
    ai.sort();
    ai.iter().for_each(|&x| println!("{}", x.1 + 1));
}
