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
fn main() {
    input! {//
        n:usize,m:usize,l:i64,
        abc:[(Usize1,Usize1,i64);m],
        q:usize,
        st:[(Usize1,Usize1);q],
    }
    let mut g = vec![vec![INF; n]; n];
    for &(a, b, c) in abc.iter() {
        if c <= l {
            g[a][b] = c;
            g[b][a] = c;
        }
    }
    let mut dp = g.clone();
    for i in 0..n {
        dp[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                chmin!(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }
    // dbg!(&dp);
    let mut anss = vec![vec![INF; n]; n];
    for i in 0..n {
        anss[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            anss[i][j] = if dp[i][j] <= l { 1 } else { INF };
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                chmin!(anss[i][j], anss[i][k] + anss[k][j]);
            }
        }
    }
    // dbg!(&anss);
    for &(s, t) in st.iter() {
        if anss[s][t] == INF {
            println!("{}", -1);
            continue;
        }
        println!("{}", anss[s][t] - 1);
    }
}
const INF: i64 = 1 << 60;
const MX: usize = 1010101;
