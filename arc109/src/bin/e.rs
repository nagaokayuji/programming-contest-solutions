#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,k:usize,
        mut s:Chars,
    }
    let mut mp = vec![vec!['x'; n]; k + 1];
    mp[1] = s.clone();
    for i in 2..k + 1 {
        let mut prv = mp[i - 1].clone();
        prv.append(&mut prv.clone());
        // dbg!(&prv);
        let cnv = |a: char| {
            if a == 'R' {
                0
            } else if a == 'S' {
                1
            } else {
                2
            }
        };
        let f = |a: char, b: char| {
            let aa = cnv(a);
            let bb = cnv(b);
            if (aa + 1) % 3 == bb {
                return a;
            }
            if (bb + 1) % 3 == aa {
                return b;
            }
            return a;
        };

        for j in 0..n {
            mp[i][j] = f(prv[2 * j], prv[2 * j + 1]);
        }
    }
    // dbg!(&mp[k]);
    let last = mp[k].clone();
    if last.len() == 1 {
        println!("{}", last[0]);
        return;
    }
    // dbg!(&last);
    let a = last[0];
    let b = last[1];

    let cnv = |a: char| {
        if a == 'R' {
            0
        } else if a == 'S' {
            1
        } else {
            2
        }
    };
    let f = |a: char, b: char| {
        let aa = cnv(a);
        let bb = cnv(b);
        if (aa + 1) % 3 == bb {
            return a;
        }
        if (bb + 1) % 3 == aa {
            return b;
        }
        return a;
    };
    let ans = f(a, b);
    println!("{}", ans);
}
