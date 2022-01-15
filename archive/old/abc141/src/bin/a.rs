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
        s:Chars
    }
    let ans = match s[0] {
        'S' => "Cloudy",
        'C' => "Rainy",
        _ => "Sunny",
    };
    println!("{}", ans);
}
