#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    https://www.youtube.com/watch?v=JYLI4mZH-p8&ab_channel=AtCoderLive
    ■
    辺: 1/出次数 で通る
    1. DAG → DP
*/
fn main() {
    input! {//
        n:usize,m:usize,
        st:[(Usize1,Usize1);m],
    }
    // 隣接リスト
    let mut to = vec![vec![]; n];
    for &(s, t) in st.iter() {
        to[s].push(t);
    }

    // ===
    // dp[n-1] = 0
    let mut dp = vec![0f64; n];
    // DAG 逆から
    for v in (0..n - 1).rev() {
        let mut now = 0f64;
        for &u in to[v].iter() {
            // 期待値の和をとる
            now += dp[u];
        }
        now /= to[v].len() as f64;
        now += 1f64;
        // 期待値
        dp[v] = now;
    }
    let mut ans = dp[0];
    fn pr(x: i64) {
        println!("{}", x);
    }
    for nv in 0..n - 1 {
        // 切ってはいけない
        if to[nv].len() == 1 {
            continue;
        }
        // 行き先のDPの値が小さいやつを探す
        let mut nu = to[nv][0];
        for &u in to[nv].iter() {
            if dp[nu] < dp[u] {
                nu = u;
            }
        }
        let mut now = {
            let mut dp = vec![0f64; n];
            for v in (0..n - 1).rev() {
                let mut now = 0f64;
                for &u in to[v].iter() {
                    // 切った辺は使わない
                    if (v == nv) && (u == nu) {
                        continue;
                    }
                    now += dp[u];
                }
                let mut deg = to[v].len();
                if v == nv {
                    deg -= 1;
                }
                now /= deg as f64;
                now += 1f64;
                dp[v] = now;
            }
            dp[0]
        };
        if ans > now {
            ans = now;
        }
    }
    println!("{}", ans);
}
