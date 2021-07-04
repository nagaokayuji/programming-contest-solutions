#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        // a:i64,b:i64,c:i64,d:i64,
        a:[i64;4],
    }
    pub fn yn(ans: bool) {
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
    let mut ok = false;
    for i in 0..1 << 4 {
        let mut x = 0;
        let mut y = 0;
        for j in 0..4 {
            if i >> j & 1 == 1 {
                x += a[j];
            } else {
                y += a[j];
            }
        }
        if x == y {
            ok = true;
        }
    }
    yn(ok);
}
