#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:Chars
    }
    let c = n.iter().filter(|&&x| x == 'R').count();
    if c == 3 {
        println!("{}", 3);
    } else if c == 2 {
        if n[1] == 'R' {
            println!("{}", 2);
        } else {
            println!("{}", 1);
        }
    } else if c >= 1 {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}
