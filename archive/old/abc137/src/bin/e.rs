#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,p:i64,
        mut abc:[(Usize1,Usize1,i64);m],
    }
    let mut edges = Vec::new();
    for (from, to, cost) in abc.iter() {
        edges.push((from, to, cost - p));
    }
    let mut dist = vec![-INF; n];
    dist[0] = 0;
    let mut ret = 0;
    for lp in 0..=n * 3 {
        for (from, to, cost) in edges.iter().cloned() {
            if dist[*from] != -INF && dist[*to] < dist[*from] + cost {
                dist[*to] = dist[*from] + cost;
                if lp > n {
                    dist[*to] = INF;
                }
            }
        }
        if lp == n - 1 {
            ret = dist[n - 1];
        }
    }
    if dist[n - 1] != ret {
        println!("{}", -1);
        return;
    }

    println!("{}", max(0, dist[n - 1]));
}
const INF: i64 = 1 << 60;
