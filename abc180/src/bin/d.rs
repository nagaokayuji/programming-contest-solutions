#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        x:i128,y:i128,a:i128,b:i128,
    }
    let y = y - 1;
    if b == 1 {
        println!("{}", max(0, y - x));
        return;
    }
    if x + b > y && x * a > y {
        println!("{}", 0);
        return;
    }
    let mut now = x;
    let mut ans = 0i128;
    while now.saturating_mul(a) <= now + b {
        // dbg!((now * a, now + b));
        if now.saturating_mul(a) > y {
            println!("{}", ans);
            return;
        }
        now = now.saturating_mul(a);
        ans += 1;
    }
    let left = y - now;
    ans += (left) / b;
    println!("{}", ans);
    return;

    let mut ok = 0;
    let mut ng = 0;
}
