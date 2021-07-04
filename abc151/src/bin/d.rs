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
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut g = vec![vec![INF; w * h]; h * w];
    let dx = vec![-1, 0, 0, 1];
    let dy = vec![0, -1, 1, 0];
    let conv = |i: usize, j: usize| i + h * j;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            g[conv(i, j)][conv(i, j)] = 0;

            for (&dx, &dy) in dx.iter().zip(dy.iter()) {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if (0..h as i32).contains(&nx) && (0..w as i32).contains(&ny) {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[nx][ny] == '.' {
                        g[conv(i, j)][conv(nx, ny)] = 1;
                    }
                }
            }
        }
    }
    for k in 0..h * w {
        for i in 0..h * w {
            for j in 0..h * w {
                g[i][j] = min(g[i][j], g[i][k] + g[k][j]);
            }
        }
    }
    let mut ans = 0i64;
    for i in 0..h * w {
        for j in 0..h * w {
            if g[i][j] != INF {
                ans = max(ans, g[i][j]);
            }
        }
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
