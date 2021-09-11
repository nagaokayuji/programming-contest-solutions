#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■ https://www.youtube.com/watch?v=9MphwmIsO7Q&ab_channel=AtCoderLive
*/
fn f(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0; n];
    for i in 0..n {
        res[i] = a[i] ^ a[(i + 1) % n];
    }
    res
}
fn main() {
    input! {//
        n:usize,
        mut a:[i64;n],
        mut b:[i64;n],
    }
    let mut xa = f(&a);
    let mut xb = f(&b);
    xb.push(-INF);
    xb.append(&mut xa.clone());
    xb.append(&mut xa.clone());
    // dbg!(&xb);
    let zlg = zalgo(&xb);
    // dbg!(&zlg);
    let mut inds = zlg
        .iter()
        .enumerate()
        .filter(|(i, &x)| x == n)
        .map(|(i, &x)| i - n - 1)
        .collect::<Vec<usize>>();
    inds.sort();
    // dbg!(&inds);
    for &k in inds.iter() {
        if k >= n {
            continue;
        }
        let x = a[k] ^ b[0];
        println!("{} {}", k, x);
    }
}
const INF: i64 = 1 << 60;
fn zalgo(s: &[i64]) -> Vec<usize> {
    let n = s.len();
    let mut a = vec![0usize; n];
    let mut from = 0;
    let mut last = 0;
    a[0] = n;
    for i in 1..n {
        let mut same = a[i];
        if from != 0 {
            same = min(a[i - from], if last > i { last - i } else { 0 });
        }
        while i + same < n && s[same] == s[same + i] {
            same += 1;
        }
        if last < i + same {
            last = i + same;
            from = i;
        }
        a[i] = same;
    }
    return a;
}
