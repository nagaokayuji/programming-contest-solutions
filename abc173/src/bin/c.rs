#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    h:usize,
    w:usize,
    k:usize,
    c : [Chars;h],
     }
    // all!
    // let mut rowc = vec![0; h];
    // let colc = vec![0; w];
    // for i in 0..h {
    //     for j in 0..w {
    //         rowc[i] += if c[i][j] == '#' { 1 } else { 0 };
    //     }
    // }
    // for i in 0..w {
    //     for j in 0..h {
    //         colc[i] += if c[j][i] == '#' { 1 } else { 0 };
    //     }
    // }
    let mut ans = 0;

    for hh in 0..1 << h {
        for ww in 0..1 << w {
            let mut blackc = 0;
            for i in 0..h {
                for j in 0..w {
                    if hh >> i & 1 == 1 && ww >> j & 1 == 1 {
                        if c[i][j] == '#' {
                            blackc += 1;
                        }
                    }
                }
            }
            if blackc == k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
