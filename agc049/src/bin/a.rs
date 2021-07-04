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
        s:[Chars;n],
    }
    let mut g = vec![vec![]; n];
    let mut rg = vec![vec![]; n];
    for (i, s) in s.clone().iter().enumerate() {
        for j in 0..n {
            if s[j] == '1' {
                g[i].push(j);
                rg[j].push(i);
            }
        }
    }
    let mut cnts = vec![0i64; n];
    // そのへん＋逆
    for i in 0..n {
        cnts[i] += 1;
        let mut q = VecDeque::new();
        let mut visited = vec![false; n];
        q.push_back(i);
        visited[i] = true;
        while let Some(q_) = q.pop_front() {
            let now = q_;
            for &nx in g[now].iter() {
                if !visited[nx] {
                    q.push_back(nx);
                    visited[nx] = true;
                    cnts[nx] += 1;
                }
            }
        }
    }
    let mut ans = 0f64;
    for i in 0..n {
        ans += 1f64 / cnts[i] as f64;
    }
    println!("{}", ans);
}
