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
        n:usize,k:i64,
        t:[[i64;n];n],
    }
    // n!
    let mut g = G {
        n: n,
        k: k,
        t: t,
        cnt: 0,
    };
    // for i in 0..n {
    //     g.rec(i, 0, 1 << i);
    // }
    g.rec(0, 0, 1);
    println!("{}", g.cnt);
}
struct G {
    n: usize,
    k: i64,
    t: Vec<Vec<i64>>,
    cnt: i64,
}
impl G {
    fn rec(&mut self, now: usize, distance: i64, visited: i64) {
        if visited.count_ones() as usize >= self.n {
            let back = distance + self.t[now][0];
            if back == self.k {
                self.cnt += 1;
            }
            return;
        }

        for i in 0..self.n {
            if visited >> i & 1 == 0 {
                self.rec(i, distance + self.t[now][i], visited | 1 << i);
            }
        }
    }
}
