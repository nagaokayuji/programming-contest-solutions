#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

macro_rules! chmin {
    ($ base : expr ,$ cmp : expr ) => {
        $base = std::cmp::min($base, $cmp);
    };
}
#[fastout]
fn main() {
    input! {//
        n:usize,
        mut xyp:[(i64,i64,i64);n],
    }
    let mut xsel = vec![vec![0; n]; 1 << n];
    let mut ysel = vec![vec![0; n]; 1 << n];
    for i in 0..1 << n {
        for j in 0..n {
            xsel[i][j] = xyp[j].0.abs();
            ysel[i][j] = xyp[j].1.abs();
            for k in 0..n {
                if (i >> k) & 1 == 1 {
                    chmin!(xsel[i][j], (xyp[j].0 - xyp[k].0).abs());
                    chmin!(ysel[i][j], (xyp[j].1 - xyp[k].1).abs());
                }
            }
        }
    }
    for i in 0..1 << n {
        for j in 0..n {
            xsel[i][j] *= xyp[j].2;
            ysel[i][j] *= xyp[j].2;
        }
    }
    let mut ans = vec![INF; n + 1];
    for i in 0..1usize << n {
        let cnt = i.count_ones() as usize;
        let mut j = i;
        loop {
            j &= i;
            let mut cost = 0;
            for k in 0..n {
                if i >> k & 1 == 0 {
                    cost += min(xsel[j][k], ysel[i - j][k]);
                }
            }
            chmin!(ans[cnt], cost);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
    for &x in ans.iter() {
        println!("{}", x);
    }
}
const INF: i64 = 1 << 60;
