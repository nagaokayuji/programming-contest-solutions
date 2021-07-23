use proconio::{marker::*, *};
use std::cmp::*;
use std::collections::*;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n],
    }
    let mut cs = vec![0; n];
    cs[0] = a[0];
    for i in 1..n {
        cs[i] = cs[i - 1] + a[i];
    }
    let mut ans = 1e18 as i64;
    for ii in 0..2 {
        let mut sum = 0i64;
        let mut sign = ii == 0;
        let mut a1 = 0i64;
        for &x in a.iter() {
            sum += x;
            if sign && sum <= 0 {
                // 1 にする
                a1 += 1 - sum;
                sum = 1;
            }
            if !sign && sum >= 0 {
                // -1 にする
                a1 += sum + 1;
                sum = -1;
            }
            sign = !sign;
        }
        ans = ans.min(a1);
    }
    println!("{}", ans);
}
