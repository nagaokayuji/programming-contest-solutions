#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,u:Usize1,v:Usize1,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![Vec::new(); 202020];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }

    let mut fromu = vec![0; 202020];
    let mut fromv = vec![0; 202020];

    let mut q = VecDeque::new();
    q.push_back((u, 0));
    while let Some(iq) = q.pop_front() {
        let mut now = iq.0;
        let mut dist = iq.1;
        fromu[now] = dist;
        for &nx in g[now].iter() {
            if fromu[nx] == 0 && nx != u {
                q.push_back((nx, dist + 1));
            }
        }
    }
    dbg!("here");
    q.clear();
    q.push_back((v, 0));
    while let Some(iq) = q.pop_front() {
        let mut now = iq.0;
        let mut dist = iq.1;
        fromv[now] = dist;
        for &nx in g[now].iter() {
            if fromv[nx] == 0 && nx != v {
                q.push_back((nx, dist + 1));
            }
        }
    }
    dbg!("heres");
    let mut ma = 0;
    for i in 0..n {
        if fromu[i] < fromv[i] {
            ma = max(ma, fromv[i] - 1);
        }
    }
    println!("{}", ma);
}
