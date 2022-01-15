#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,m:usize,
    }

    let mut v = vec![vec![0; n]; m]; //電球iと関連のあるswitch j
    for i in 0..m {
        input! {
        k:usize,
        s:[Usize1;k],}
        for j in 0..k {
            v[i][s[j]] = 1;
        }
    }
    input! {
        p:[usize;m],
    }

    let mut ans = 0;
    for switch_b in 0..1 << n {
        let mut ok = true;
        for denkyu in 0..m {
            let mut cnt = 0;
            for switch in 0..n {
                if switch_b >> switch & 1 == 1 {
                    cnt += v[denkyu][switch];
                }
            }
            if p[denkyu] != cnt % 2 {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
