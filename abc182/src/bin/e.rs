#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        h:usize,w:usize,n:usize,m:usize,
        mut ab:[(Usize1,Usize1);n],
        mut cd:[(Usize1,Usize1);m],
    }
    // dp?
    let mut dp = vec![vec![0; w]; h];
    for &(a, b) in ab.iter() {
        dp[a][b] = 3;
    }
    for &(c, d) in cd.iter() {
        dp[c][d] = 4;
    }
    for i in 0..h {
        for j in 0..w {
            if dp[i][j] & 4 == 4 {
                continue;
            }
            // よこ
            if dp[i][j] & 4 == 0 && dp[i][j] & 1 == 1 {
                if j + 1 < w {
                    if dp[i][j + 1] & 4 == 0 {
                        dp[i][j + 1] |= 1;
                    }
                }
                if j > 0 {
                    if dp[i][j - 1] & 4 == 0 {
                        dp[i][j - 1] |= 1;
                    }
                }
            }
        }
        for j in (0..w).rev() {
            if dp[i][j] & 4 == 4 {
                continue;
            }
            // よこ
            if dp[i][j] & 4 == 0 && dp[i][j] & 1 == 1 {
                if j + 1 < w {
                    if dp[i][j + 1] & 4 == 0 {
                        dp[i][j + 1] |= 1;
                    }
                }
                if j > 0 {
                    if dp[i][j - 1] & 4 == 0 {
                        dp[i][j - 1] |= 1;
                    }
                }
            }
        }
    }
    // dbg!(&dp);
    for j in 0..w {
        for i in 0..h {
            if dp[i][j] & 4 == 4 {
                continue;
            }
            // たて
            if dp[i][j] & 4 == 0 && dp[i][j] & 2 == 2 {
                if i + 1 < h {
                    if dp[i + 1][j] & 4 == 0 {
                        dp[i + 1][j] |= 2;
                    }
                }
                if i > 0 {
                    if dp[i - 1][j] & 4 == 0 {
                        dp[i - 1][j] |= 2;
                    }
                }
            }
        }
        for i in (0..h).rev() {
            if dp[i][j] & 4 == 4 {
                continue;
            }
            // たて
            if dp[i][j] & 4 == 0 && dp[i][j] & 2 == 2 {
                if i + 1 < h {
                    if dp[i + 1][j] & 4 == 0 {
                        dp[i + 1][j] |= 2;
                    }
                }
                if i > 0 {
                    if dp[i - 1][j] & 4 == 0 {
                        dp[i - 1][j] |= 2;
                    }
                }
            }
        }
    }
    // dbg!(&dp);
    let mut ans = 0i64;
    // for i in 0..h {
    //     for j in 0..w {
    //         if dp[i][j] & 4 == 0 {
    //             if i > 0 && dp[i - 1][j] & 4 == 0 && dp[i - 1][j] & 1 == 1 {
    //                 dp[i][j] |= 1;
    //             }
    //             if j > 0 && dp[i][j - 1] & 4 == 0 && dp[i][j - 1] & 2 == 2 {
    //                 dp[i][j] |= 2;
    //             }
    //         }
    //     }
    // }
    // dbg!(&dp);
    for i in 0..h {
        for j in 0..w {
            if dp[i][j] & 4 == 0 && dp[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
