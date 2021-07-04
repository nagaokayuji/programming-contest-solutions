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
*/
#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
        mut a:[i64;n],
        mut xy:[(Usize1,Usize1);m],
    }
    if m == 1 {
        println!("{}", a[xy[0].1] - a[xy[0].0]);
        return;
    }
    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for (i, &(x, y)) in xy.iter().enumerate() {
        g[x].push((y, a[y] - a[x], i));
        rg[y].push(x);
    }

    let mut ans = -INF;
    let mut dp = vec![INF; n];
    for (i, x) in rg.iter().cloned().enumerate() {
        for &u in x.iter() {
            chmin!(dp[i], dp[u], a[u]);
        }
    }
    for i in 0..n {
        chmax!(ans, a[i] - dp[i]);
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
