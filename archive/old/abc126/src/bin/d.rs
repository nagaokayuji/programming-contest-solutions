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
        n:usize,
        uvw:[(Usize1,Usize1,i64);n-1],
    }

    let root = 0;
    let mut g = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut color = vec![false; n];
    color[0] = true;

    let mut stack = vec![];
    stack.push((0, !0, 0));
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(qq) = stack.pop() {
        let (now, prev, dist) = qq;
        if prev != !0 {
            color[now] = if dist % 2 == 0 {
                color[prev]
            } else {
                !color[prev]
            };
        }
        for &(nx, d) in g[now].clone().iter() {
            if visited[nx] {
                continue;
            }
            visited[nx] = true;
            stack.push((nx, now, d));
        }
    }
    color
        .iter()
        .for_each(|&x| println!("{}", if x { 1 } else { 0 }));
}
