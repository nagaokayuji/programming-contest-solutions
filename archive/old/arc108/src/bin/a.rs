#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        s:i64,p:i64,
    }
    let s = s as usize;
    let dv = divisor(p as usize);

    for &x in dv.iter() {
        let (n, m) = (x, p as usize / x);
        if n + m == s {
            yn(true);
            return;
        }
    }
    yn(false);
    fn yn(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
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
