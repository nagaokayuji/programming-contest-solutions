#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
#[fastout]
fn main() {
    input! {//
        x:f64,y:f64,r:f64,
    }
    let x = (x * 1e4).round() as i128;
    let y = (y * 1e4).round() as i128;
    let r = (r * 1e4).round() as i128;
    let unit = 1e4 as i128;
    let x = x - x / unit * unit;
    let y = y - y / unit * unit;
    let y = y + unit * 1e6 as i128;
    let x = x + unit * 1e6 as i128;
    let mut ans = 0i128;
    let mut xx = unit * ((x - r) / unit);
    assert!(xx >= 0);
    while xx <= x + r {
        let p = r * r - (x - xx) * (x - xx);
        let binsearch = |t: i128| {
            let isok = |mid: i128| mid * mid <= p as i128;
            let mut ok = 0i128;
            let mut ng = 1e14 as i128;

            while ng - ok > 1 {
                let mid = (ok + ng) / 2;
                if isok(mid) {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            ok
        };
        let b = binsearch(p as i128) as i128;
        let u = (b + y) / unit;
        let l = (-b + y + unit - 1) / unit;
        ans += u - l + 1;
        xx += unit;
    }
    println!("{}", ans);
}
