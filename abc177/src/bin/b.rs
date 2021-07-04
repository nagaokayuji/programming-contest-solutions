#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        s:Chars,
        t:Chars,
    }

    let mut mx = INF;
    for i in 0..s.len() {
        let mut dif = 0;
        for j in 0..t.len() {
            if i + j >= s.len() {
                dif = INF;
                break;
            }
            if s[i + j] != t[j] {
                dif += 1;
            }
        }
        mx = min(mx, dif);
    }
    println!("{}", mx);
}

const INF: i64 = 1 << 60;
