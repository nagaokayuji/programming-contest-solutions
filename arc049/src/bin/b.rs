#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
fn main() {
    input! {//
        n:usize,
        mut xyc:[(f64,f64,f64);n],
    }
    let mut ng = 0f64;
    let mut ok = 1e10;
    let isOk = |t: f64| -> bool {
        let mut xmn = -1e9;
        let mut xmx = 1e9;
        let mut ymn = -1e9;
        let mut ymx = 1e9;
        for &(x, y, c) in xyc.iter() {
            let to = t / c;
            chmax!(xmn, x - to);
            chmax!(ymn, y - to);
            chmin!(xmx, x + to);
            chmin!(ymx, y + to);
        }
        return xmn <= xmx && ymn <= ymx;
    };
    while (ok - ng) > 1e-5 {
        let mid = (ok + ng) / 2f64;
        if isOk(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
