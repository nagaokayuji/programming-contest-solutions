#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
    A_1 = X, A_{i+1} = A_i + D
    等差数列？
    適当に選んだ和→S、残り→T
    S-Tが何通りか

    https://www.youtube.com/watch?v=tNyPYIhy9Ms&feature=youtu.be&ab_channel=AtCoderLive
*/
fn main() {
    input! {//
        n:usize,
        mut x:i64,mut d:i64,
    }
    if d == 0 {
        if x == 0 {
            println!("{}", 1);
        } else {
            println!("{}", n + 1);
        }
        return;
    }
    let mut mp = BTreeMap::new();
    for i in 0..=n as i64 {
        let s = x * i;
        let l = i * (i - 1) / 2;
        let r = l + i * (n as i64 - i);
        let s = s + l * d;
        let r = r - l;
        let md = (s % d + d) % d;
        mp.entry(md)
            .or_insert(Vec::<(i64, i64)>::new())
            .push((s, r));
    }
    let mut ans = 0i64;
    for (key, value) in mp {
        let mut a = value.clone();
        let m = value.len();
        for i in 0..m {
            a[i].0 = (a[i].0 - key) / d;
        }
        let mut ev = vec![];
        for i in 0..m {
            ev.push((a[i].0, 1));
            ev.push((a[i].0 + a[i].1 + 1, -1));
        }
        ev.sort();
        let mut cnt = 0;
        let mut pre = -INF;
        for &e in ev.iter() {
            let len = e.0 - pre;
            if cnt > 0 {
                ans += len;
            }
            cnt += e.1;
            pre = e.0;
        }
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;
