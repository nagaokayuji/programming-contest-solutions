#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

fn main() {
    input! {//
        n:usize,
        s:Chars,
    }
    s.iter().for_each(|&x| {
        print!(
            "{}",
            ((((x as usize - b'A' as usize) + n) % 26) as u8 + b'A') as char
        )
    });
}
