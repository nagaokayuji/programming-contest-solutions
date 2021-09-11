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
        n:usize, m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
    }

    let mut shortest = INF;
    let mut res = vec![];
    for s in 0..n {
        let mut dist = vec![INF; n];
        let mut prev = vec![None; n];
        let mut que = VecDeque::new();
        que.push_back(s);
        dist[s] = 0;
        while let Some(now) = que.pop_front() {
            for &nx in g[now].iter() {
                if chmin!(dist[nx], dist[now] + 1) {
                    prev[nx] = Some(now);
                    que.push_back(nx);
                }
            }
        }
        for t in 0..n {
            if t == s || dist[t] == INF {
                continue;
            }
            for &nx in g[t].iter() {
                if nx == s {
                    // 復元
                    let mut tmp = vec![s];
                    let mut cur = t;
                    loop {
                        tmp.push(cur);
                        if let Some(prv) = prev[cur] {
                            cur = prv;
                        } else {
                            break;
                        }
                        if cur == s {
                            break;
                        }
                    }
                    if chmin!(shortest, tmp.len() as i64) {
                        res = tmp;
                    }
                }
            }
        }
    }
    // dbg!(&tmp);
    if shortest == INF {
        println!("{}", -1);
        return;
    }
    println!("{}", res.len());
    res.iter().for_each(|&x| println!("{}", x + 1));
}
const INF: i64 = 1 << 60;
