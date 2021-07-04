#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        h:usize,w:usize,
        s:[Chars;h],
    }

    let mut yoko = vec![vec![0; w]; h];
    let mut tate = vec![vec![0; w]; h];

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                if j == 0 {
                    yoko[i][j] = 1;
                } else {
                    yoko[i][j] += yoko[i][j - 1] + 1;
                }
            }
        }
        for j in (0..w).rev() {
            if j == w - 1 {
                continue;
            } else {
                if yoko[i][j] != 0 {
                    yoko[i][j] = max(yoko[i][j], yoko[i][j + 1]);
                }
            }
        }
    }
    for j in 0..w {
        for i in 0..h {
            if s[i][j] == '.' {
                if i == 0 {
                    tate[i][j] = 1;
                } else {
                    tate[i][j] += tate[i - 1][j] + 1;
                }
            }
        }
        for i in (0..h).rev() {
            if i == h - 1 {
                continue;
            } else {
                if tate[i][j] != 0 {
                    tate[i][j] = max(tate[i][j], tate[i + 1][j]);
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = max(ans, tate[i][j] + yoko[i][j] - 1);
        }
    }
    println!("{}", ans);
}
