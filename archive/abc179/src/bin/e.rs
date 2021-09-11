#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:i64,
        x:i64,
        MOD:i64,
    }
    let mut a = x;
    let mut v = vec![];
    v.push(a);
    let mut dup = 0;
    for k in 2..=1234124111 {
        a = (a * a) % MOD;
        if v.contains(&a) {
            dup = a;
            break;
        }
        v.push(a);
    }
    let ln = v.len() as i64;
    let mut ans = 0i64;
    let mut tt = 0i64;
    let mut dd = 0i64;
    if n < ln {
        ans = v.iter().take(n as usize).fold(0, |sum, x| sum + x);
        println!("{}", ans);
        return;
    }
    dd = v.iter().fold(0, |sum, x| sum + x);
    ans += dd;
    let n = n - ln;
    let mut a = dup;
    let mut v = vec![];
    v.push(a);
    for ll in 1.. {
        a = (a * a) % MOD;
        if v.contains(&a) {
            break;
        }
        v.push(a);
    }
    // dbg!(&v);
    let ln = v.len() as i64;
    if n < ln {
        ans += v.iter().take(n as usize).fold(0, |sum, x| sum + x);
        println!("{}", ans);
        return;
    }
    for (i, &x) in v.iter().enumerate() {
        let i = (i) as i64;
        ans += x * ((n - i + ln - 1) / ln);
    }
    println!("{}", ans);
}
