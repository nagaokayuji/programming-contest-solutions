#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
#[fastout]
fn main() {
    input! {//
        n:i64,m:usize,
        mut a:[i64;m],
    }
    if n == m as i64 {
        println!("{}", 0);
        return;
    }
    if m == 0 {
        println!("{}", 1);
        return;
    }
    a.sort();

    // kmax=gcd
    if a[0] != 1 {
        a.push(0)
    }
    if a[m - 1] != n {
        a.push(n + 1);
    }
    a.sort();
    let m = a.len();
    // 幅k
    let mut k = INF;
    for i in 1..m {
        let koho = a[i] - a[i - 1] - 1;
        if koho > 0 {
            chmin!(k, koho);
        }
    }
    let mut ans = 0;
    for i in 1..m {
        let haba = a[i] - a[i - 1] - 1;
        ans += (haba + k - 1) / k;
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
