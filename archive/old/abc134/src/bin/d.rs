#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
       mut a:[i64;n],
    }
    // let mut a = vec![0];
    // a.append(&mut b);
    let mut anss = vec![false; n + 1];
    let mut z = vec![];
    for i in (0..n).rev() {
        let j = divisor(i + 1);
        if ((a[i] == 1) ^ anss[i + 1]) {
            for divs in j {
                anss[divs] = !anss[divs];
            }
            z.push(i + 1);
        }
    }
    println!("{}", z.len());
    for &x in z.iter() {
        println!("{}", x);
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
