#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
#[fastout]
fn main() {
    input! {//
        h:usize,w:usize,
        a:[Chars;h],
    }
    let mut st = (0, 0);
    let mut gl = (0, 0);
    let mut dp = vec![vec![INF; w + 1]; h + 1];
    let mut zahyo = vec![vec![]; 26];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                st = (i, j);
                dp[i][j] = 0;
            } else if a[i][j] == 'G' {
                gl = (i, j);
            } else if a[i][j] != '#' && a[i][j] != '.' {
                let d = (a[i][j] as u8 - b'a') as usize;
                zahyo[d].push((i, j));
            }
        }
    }
    let mut q = VecDeque::new();
    q.push_back(st);
    let dx = vec![-1, 0, 0, 1];
    let dy = vec![0, -1, 1, 0];
    let mut used = vec![false; 26];
    while let Some(q_) = q.pop_front() {
        let now = q_;
        if ('a'..='z').contains(&a[now.0][now.1]) {
            let d = (a[now.0][now.1] as u8 - b'a') as usize;
            if !used[d] {
                for &nx in zahyo[d].iter() {
                    if chmin!(dp[nx.0][nx.1], dp[now.0][now.1] + 1) {
                        q.push_front(nx);
                    }
                }
                used[d] = true;
            }
        }
        for i in 0..4 {
            let (dx, dy) = (dx[i], dy[i]);
            let (nx, ny) = (now.0 as i64 + dx, now.1 as i64 + dy);
            if (0..h as i64).contains(&nx) && (0..w as i64).contains(&ny) {
                let nx = nx as usize;
                let ny = ny as usize;
                if a[nx][ny] == '#' {
                    continue;
                }
                if chmin!(dp[nx][ny], dp[now.0][now.1] + 1) {
                    // next
                    q.push_back((nx, ny));
                }
            }
        }
    }
    if dp[gl.0][gl.1] == INF {
        println!("{}", -1);
    } else {
        let ans = dp[gl.0][gl.1];
        println!("{}", ans);
    }
}
const INF: i64 = 1 << 60;
