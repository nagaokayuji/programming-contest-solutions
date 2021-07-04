#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:i64,
     }
    // 2,5,10
    if n % 2 == 1 {
        println!("{}", 0);
        return;
    } else {
        let mut ans = 0;
        let mut dd = 10;
        for i in 0..333333333 {
            ans += n / dd;
            dd *= 5;
            if n / dd == 0 {
                break;
            }
        }
        println!("{}", ans);
    }
}
