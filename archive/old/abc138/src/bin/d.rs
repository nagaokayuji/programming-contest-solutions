#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        n:usize,q:usize,
        ab:[(Usize1,Usize1);n-1],
        mut px:[(Usize1,i64);q],
    }
    px.sort();
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut root = 0;
    let mut ad = vec![0i64; n];
    for &(p, x) in px.iter() {
        ad[p] += x;
    }
    let mut ans = vec![0i64; n];
    let mut q = VecDeque::new();
    let mut add = 0i64;
    let mut visited = vec![false; n];
    visited[0] = true;
    q.push_back((0, 0));
    while let Some(qq) = q.pop_back() {
        let (now, mut add_now) = qq;
        visited[now] = true;
        add_now += ad[now];
        ans[now] += add_now;
        for &nx in g[now].iter() {
            if visited[nx] {
                continue;
            }
            q.push_back((nx, add_now));
        }
    }
    ans.iter().for_each(|&x| println!("{}", x));
}
