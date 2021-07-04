#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

const INF: i64 = 1 << 60;
#[fastout]
fn main() {
    input! {
    n:usize,k:usize,c:i64,
    s:Chars,
     }

    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut prev = -INF;
    for (i, &now) in s.iter().enumerate() {
        let i = i as i64;
        if left.len() >= k {
            break;
        }
        if now == 'o' {
            if i - prev > c {
                prev = i;
                left.push(i);
            }
        }
    }
    let mut prev = INF;
    for (i, &now) in s.iter().enumerate().rev() {
        let i = i as i64;
        if right.len() >= k {
            break;
        }
        if now == 'o' {
            if prev - i > c {
                prev = i;
                right.push(i);
            }
        }
    }
    right.sort();
    for (i, &l) in left.iter().enumerate() {
        if right[i] == l {
            println!("{}", l + 1);
        }
    }
}
