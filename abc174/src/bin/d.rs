#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
      mut  c:Chars,
    }
    let mut lr = 0;
    let mut lw = 0;
    let mut rw = 0;
    let mut rr = 0;
    for &x in c.iter() {
        if x == 'W' {
            rw += 1;
        } else {
            rr += 1;
        }
    }
    let mut ans = min(rw, rr);
    for i in 0..n {
        if c[i] == 'W' {
            lw += 1;
            rw -= 1;
        } else {
            lr += 1;
            rr -= 1;
        }
        ans = min(ans, (lw + rr + 1) / 2);
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
