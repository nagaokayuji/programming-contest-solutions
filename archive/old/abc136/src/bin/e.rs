#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:usize,
        a:[usize;n],
    }
    let mut sm = a.iter().fold(0, |sum, x| sum + x);
    let mut dvs = divisor(sm);

    dvs.sort_by_key(|&x| -(x as i64));

    for &d in dvs.iter() {
        let mut nms = vec![];
        for &x in a.iter() {
            nms.push(x % d);
        }
        nms.sort();
        let mut c1 = 0;
        let mut c2 = 0;
        let mut r = n - 1;
        for l in 0..n {
            c1 += nms[l];
            while c1 > c2 {
                c2 += d - nms[r];
                r -= 1;
            }
            if l >= r {
                break;
            }
        }
        if c1 <= k {
            println!("{}", d);
            return;
        }
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
