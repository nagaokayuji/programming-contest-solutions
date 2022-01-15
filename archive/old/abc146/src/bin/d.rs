#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (ind, &(a, b)) in ab.iter().enumerate() {
        g[a].push((b, ind));
        g[b].push((a, ind));
    }

    let mut q = VecDeque::new();
    q.push_back((0, !0, !0));
    let mut ans = vec![0; n - 1];
    while let Some(q_) = q.pop_front() {
        let (now, col, prv) = q_;
        let mut nowc = if col == 1 { 2 } else { 1 };
        // ans[now] = col;
        for &nx in g[now].iter() {
            if nx.0 == prv {
                continue;
            }
            q.push_back((nx.0, nowc, now));
            ans[nx.1] = nowc;
            nowc = if nowc + 1 == col { nowc + 2 } else { nowc + 1 };
        }
    }
    // println!("{}", ans.len());
    println!("{}", ans.iter().max().unwrap());
    for &x in ans.iter() {
        println!("{}", x);
    }
}
