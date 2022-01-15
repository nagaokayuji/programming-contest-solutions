#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
        uv:[(Usize1,Usize1);m],
        s:Usize1,t:Usize1,
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        g[u].push(v);
    }

    let mut dist = vec![vec![!0; 3]; n];
    dist[s][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some(x) = q.pop_front() {
        let (v, p) = x;
        for &nx in g[v].iter() {
            let pp = (p + 1) % 3;
            if dist[nx][pp] == !0 {
                dist[nx][pp] = dist[v][p] + 1;
                q.push_back((nx, pp));
            }
        }
    }
    if dist[t][0] != !0 {
        println!("{}", dist[t][0] / 3);
    } else {
        println!("{}", -1);
    }
}
