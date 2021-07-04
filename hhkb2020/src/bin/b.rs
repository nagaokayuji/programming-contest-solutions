#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        h:usize,
        w:usize,
        s:[Chars;h],
    }
    let mut ans = 0i64;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                if s[i][j] == '.' && s[i + 1][j] == '.' {
                    ans += 1;
                }
            }
            if j + 1 < w {
                if s[i][j] == '.' && s[i][j + 1] == '.' {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
