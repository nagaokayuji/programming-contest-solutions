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
        n:usize,m:usize,
        mut abc:[(Usize1,Usize1,i64);m],
    }
    let mut ans = 0i64;

    let mut dir = vec![vec![INF; n]; n];
    for &(a, b, c) in abc.iter() {
        dir[a][b] = c;
    }
    let mut dp = vec![vec![INF; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    // abc  wo max(a,b) de sort sitai

    abc.sort_by_key(|&(a, b, c)| a.max(b));
    dbg!(&abc);

    let mut i = 0usize;
    for k in 0..n {
        while i < m && max(abc[i].0, abc[i].1) <= k {
            let aa = abc[i].0;
            let bb = abc[i].1;
            dp[abc[i].0][abc[i].1] = min(dp[abc[i].0][abc[i].1], abc[i].2);
            for ii in 0..n {
                for jj in 0..n {
                    dp[ii][jj] = min(dp[ii][jj], dp[ii][aa] + dp[aa][jj]);
                    dp[ii][jj] = min(dp[ii][jj], dp[ii][bb] + dp[bb][jj]);
                }
            }
            i += 1;
        }
        for ii in 0..n {
            for jj in 0..n {
                dp[ii][jj] = min(dp[ii][jj], dp[ii][k] + dp[k][jj]);
            }
        }
        for s in 0..n {
            for t in 0..n {
                if s == t {
                    continue;
                }
                // if k > s || k > t {
                //     continue;
                // }
                let ko = min(dir[s][t], dp[s][t]);
                if ko != INF {
                    ans += ko;
                    dbg!(s, t, k, ko);
                }
            }
        }
    }
    dbg!(&dp);
    println!("{}", ans);
}

const INF: i64 = 1 << 59;
