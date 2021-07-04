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
/*
    ■
    https://atcoder.jp/contests/arc109/editorial/380
*/
fn main() {
    input! {//
        t:usize,
    }
    for _ in 0..t {
        solve();
    }
}
fn solve() {
    input! {
        mut a:(i64,i64),
        mut b:(i64,i64),
        mut c:(i64,i64),
    }

    let x = (a.0 + b.0 + c.0);
    let y = (a.1 + b.1 + c.1);

    let x = if x > 0 { x - x / 3 } else { x + (2 - x) / 3 } - 1;
    let y = if y > 0 { y - y / 3 } else { y + (2 - y) / 3 } - 1;
    if x == 0 && y == 0 {
        println!("{}", 0);
        return;
    }
    if x == 1 && y == 1 {
        println!("{}", 1);
        return;
    }
    println!("{}", max(x.abs(), y.abs()) + if (x == y) { 1 } else { 0 });
}
