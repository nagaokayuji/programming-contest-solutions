#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:i64,
        a:[i64;n],
    }
    let mut right = 0;
    let mut sum = 0;
    let mut ret = 0;
    let mut left = 0;
    while left < n {
        while right < n && sum < k {
            sum += a[right];
            right += 1;
        }
        if sum >= k {
            ret += (n + 1 - right);
        }
        sum -= a[left];
        left += 1;
    }
    println!("{}", ret);
}
