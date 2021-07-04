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
        a:[i64;n],
        b:[i64;n],
    }
    /**
     * (x_i, x_i+1) が　ペア　　<=> x_i と x_i+1 の偶奇は異なる
     * 必要十分条件
     */
    let mut odds = vec![];
    let mut evens = vec![];
    for (i, (&a, &b)) in a.iter().zip(b.iter()).enumerate() {
        let x = b - a;
        if i % 2 == 0 {
            evens.push(x);
        } else {
            odds.push(x);
        }
    }
    evens.sort();
    odds.sort();
    odds.reverse();
    evens.reverse();
    // ↓ この答えのとり方
    let mut ans = a.iter().fold(0, |sum, x| sum + x);
    let mut now = ans;
    for (&o, &e) in odds.iter().zip(evens.iter()) {
        now += o + e; // pair
        ans = max(ans, now);
    }
    println!("{}", ans);
}
