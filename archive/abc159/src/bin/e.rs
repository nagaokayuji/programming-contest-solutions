#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
/**
 * snuke code
**/
fn main() {
    input! {
            h:usize,
            w:usize,
            k:usize,
        s:[Chars;h],
    }
    // dbg!(&s);
    let mut ans = INF;
    // 横に切る方法を全探索
    for div in 0..(1usize << (h - 1)) {
        let mut g: usize = 0;
        let mut id = vec![0usize; h + 1];
        for i in 0..h {
            id[i] = g;
            if (((div >> i) & 1) == 1) {
                g += 1;
            }
        }
        g += 1;
        let mut c = vec![vec![0usize; w]; g];
        for i in 0..h {
            for j in 0..w {
                c[id[i]][j] += (s[i][j] as u8 - b'0') as usize;
            }
        }

        // dbg!(&c);
        let mut num: usize = g - 1;
        // 各グループで何個のホワイトチョコがあるか
        let mut now = vec![0usize; g];
        for j in 0..w {
            if !add(&mut now, j, g, k, &c) {
                num += 1;
                now = vec![0usize; g];
                if !add(&mut now, j, g, k, &c) {
                    num = INF;
                    break;
                }
            }
        }
        ans = min(ans, num);
    }
    println!("{}", ans);
}
// j列目をnowに加算
fn add(now: &mut Vec<usize>, j: usize, g: usize, k: usize, c: &Vec<Vec<usize>>) -> bool {
    for i in 0..g {
        now[i] += c[i][j];
    }
    for i in 0..g {
        if now[i] > k {
            return false;
        }
    }
    return true;
}
const INF: usize = 1 << 62;

pub fn charconv(c: char, base: char) -> usize {
    return (c as u8 - base as u8) as usize;
}
