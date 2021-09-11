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
    let mut n = n;
    if n == 2 {
        println!("{}", 1);
        return;
    }
    let mut ans = 0;
    let mut divs = divisor((n - 1) as usize);
    ans += divs.len() - 1;
    let mut dvs2 = divisor(n as usize);
    for &d in dvs2.iter() {
        if d == 1 {
            continue;
        }
        let mut m = n as usize;
        while m % d == 0 {
            m /= d;
        }
        if m % d == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
pub fn divisor(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in (1..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res.sort();
    res
}
