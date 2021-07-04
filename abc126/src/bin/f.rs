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
        m:usize,
        k:usize,
    }
    // 0にできる。
    if m == 0 {
        if k == 0 {
            println!("0 0");
        } else {
            println!("{}", -1);
        }
        return;
    }
    if m == 1 {
        if k == 0 {
            println!("0 0 1 1");
        } else {
            println!("{}", -1);
        }
        return;
    }
    if k as usize >= 1usize << m {
        println!("{}", -1);
        return;
    }

    // つくれる
    let mut ans = vec![0; 1usize << (m + 1)];
    let n = ans.len();
    ans[0] = k;
    let mut cnt = 0;
    for i in 1..n / 2 {
        if cnt == k {
            cnt += 1;
        }
        ans[i] = cnt;
        cnt += 1;
    }
    ans[n / 2] = k;
    let mut rcnt = n / 2;
    // dbg!(rcnt);
    for i in n / 2 + 1..n {
        rcnt -= 1;
        if rcnt == k {
            rcnt -= 1;
        }
        ans[i] = rcnt;
    }
    // dbg!(&ans);
    ans.iter().for_each(|&x| println!("{}", x));
}
