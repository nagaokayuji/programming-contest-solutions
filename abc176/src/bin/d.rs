#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

macro_rules! chmin {
    ($ base : expr , $ cmp : expr ) => {
        if $base > $cmp {
            $base = $cmp;
        }
    };
}
macro_rules! chmax {
    ($ base : expr , $ cmp : expr ) => {
        if $base < $cmp {
            $base = $cmp;
        }
    };
}
#[fastout]
fn main() {
    input! {//
        h:usize,w:usize,
        c:(Usize1,Usize1),
        d:(Usize1,Usize1),
        s:[Chars;h],
    }

    let mut dp = vec![vec![INF; w]; h];
    let mut q = VecDeque::new();
    q.push_back((c.0, c.1, 0));
    let dx = vec![-1, 0, 0, 1];
    let dy = vec![0, -1, 1, 0];
    while let Some(x) = q.pop_front() {
        let (nowh, noww, cnt) = x;
        // if dp[nowh][noww] <= cnt {
        //     continue;
        // }
        // dbg!(&x);
        dp[nowh][noww] = min(dp[nowh][noww], cnt);
        let mut flg = false;
        for o in 0..4 {
            let nxh = nowh as i64 + dx[o];
            let nxw = noww as i64 + dy[o];
            if (0..h as i64).contains(&nxh) && (0..w as i64).contains(&nxw) {
                let nxh = nxh as usize;
                let nxw = nxw as usize;
                if dp[nxh][nxw] > cnt && s[nxh][nxw] == '.' {
                    dp[nxh][nxw] = cnt;
                    q.push_front((nxh, nxw, cnt));
                    flg = true;
                }
            } else {
                continue;
            }
        }
        for hh in -2..=2 {
            for ww in -2..=2 {
                let nxh = nowh as i64 + hh;
                let nxw = noww as i64 + ww;
                if (0..h as i64).contains(&nxh) && (0..w as i64).contains(&nxw) {
                    let nxh = nxh as usize;
                    let nxw = nxw as usize;
                    if dp[nxh][nxw] > cnt + 1 && s[nxh][nxw] == '.' {
                        dp[nxh][nxw] = cnt + 1;
                        q.push_back((nxh, nxw, cnt + 1));
                    }
                }
            }
        }
        // }
    }
    // dbg!(&dp);
    if dp[d.0][d.1] == INF {
        println!("{}", -1);
    } else {
        println!("{}", dp[d.0][d.1]);
    }
}
const INF: i64 = 1 << 60;
