#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        s:Chars,
    }
    let mut cnta = vec![0; n + 1];
    let mut cntt = vec![0; n + 1];
    let mut cntg = vec![0; n + 1];
    let mut cntc = vec![0; n + 1];
    for i in 0..n {
        cnta[i + 1] = cnta[i];
        cntt[i + 1] = cntt[i];
        cntg[i + 1] = cntg[i];
        cntc[i + 1] = cntc[i];
        if s[i] == 'A' {
            cnta[i + 1] = cnta[i] + 1;
        }
        if s[i] == 'T' {
            cntt[i + 1] = cntt[i] + 1;
        }
        if s[i] == 'G' {
            cntg[i + 1] = cntg[i] + 1;
        }
        if s[i] == 'C' {
            cntc[i + 1] = cntc[i] + 1;
        }
    }
    let mut ans = 0i64;
    for i in 0..n {
        for j in i + 1..=n {
            let a = cnta[j] - cnta[i];
            let t = cntt[j] - cntt[i];
            let g = cntg[j] - cntg[i];
            let c = cntc[j] - cntc[i];
            if a == t && g == c {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
