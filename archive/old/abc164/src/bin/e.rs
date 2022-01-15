#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,s:usize,
    uvab : [(Usize1,Usize1,usize,usize);m],// city,city,fee, transit time
    cd:[(usize,usize);n],// money, time
    }
    let mut G = vec![Vec::new(); n + 1];
    for &(u, v, a, b) in uvab.iter() {
        G[u].push((v, a, b));
        G[v].push((u, a, b));
    }
    let mut s = s;
    /**
     * 銀貨は最大でも2500枚あればよい -> 制約ゆるくなる
     * */
    let MAX = 2500usize;
    if s >= MAX {
        s = MAX;
    }
    // [頂点, 金]への最大値
    let mut dp = vec![vec![INF as usize; MAX + 1]; n];
    let mut qq = BTreeSet::new();
    dp[0][s] = 0;
    // ダイクストラの拡張
    qq.insert((0, 0, s));
    while qq.len() > 0 {
        let tp = *qq.iter().next().unwrap();
        qq.remove(&tp);
        let time = (tp.0) as usize;
        let v = (tp.1) as usize;
        let ss = (tp.2) as usize;
        // 最大より時間がかかるなら無視
        if time > dp[v][ss] {
            continue;
        }
        // 両替する
        if ss + cd[v].0 <= MAX {
            let ns = ss + cd[v].0;
            let ntime = time + cd[v].1;
            if dp[v][ns] > ntime {
                dp[v][ns] = ntime;
                qq.insert((ntime, v, ns));
            }
        }
        for &e in G[v].iter() {
            if ss < e.1 {
                continue;
            }
            let nv = e.0;
            let ns = ss - e.1;
            let ntime = time + e.2;
            if dp[nv][ns] > ntime {
                dp[nv][ns] = ntime;
                qq.insert((ntime, nv, ns));
            }
        }
    }
    for v in 1..n {
        let mut res = INF as usize;
        for s in 0..=MAX {
            res = min(res, dp[v as usize][s as usize]);
        }
        println!("{}", res);
    }
}

const INF: usize = 1 << 60;
