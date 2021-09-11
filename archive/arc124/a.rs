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
        n:usize,k:usize,
       mut ck:[(char,usize);k],
    }
    let mo = 998244353i64;

    // n-k
    let fr = n - k;
    if fr == 0 {
        println!("1");
        return;
    }

    let mut ngvalues = vec![HashSet::new(); n];

    for (i, &(c, k)) in ck.iter().enumerate() {
        if c == 'L' {
            for l in 0..k {
                ngvalues[l].insert(i + 1);
            }
        } else {
            for l in k..n {
                ngvalues[l].insert(i + 1);
            }
        }
    }

    let mut ans = 1i64;
    for i in 0..n {
        let mut ok = true;
        for j in 0..k {
            if ck[j].0 == 'L' {
                if i == ck[j].1 - 1 {
                    ok = false;
                    continue;
                }
            } else {
                if i == ck[j].1 - 1 {
                    ok = false;
                    continue;
                }
            }
        }
        if !ok {
            continue;
        }
        ans *= (k - ngvalues[i].len()) as i64;
        ans %= mo;
    }
    println!("{}", ans);
}
