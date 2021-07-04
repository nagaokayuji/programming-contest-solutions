#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a:[usize;n],
        mut b:[usize;n],
    }
    let mut c = vec![0; n + 3];
    let mut d = vec![0; n + 3];
    for i in 0..n {
        c[a[i]] += 1;
        d[b[i]] += 1;
    }
    let mut x = 0i64;
    for i in 1..=n {
        if c[i] + d[i] > n {
            yn(false);
            return;
        }
        c[i] += c[i - 1];
        d[i] += d[i - 1];
        x = max(x, c[i] as i64 - d[i - 1] as i64);
    }

    yn(true);
    for i in 0..n {
        print!("{}", b[(i as i64 + n as i64 - x as i64) as usize % n]);
        assert!(a[i] != b[(i as i64 + n as i64 - x as i64) as usize % n]);
        print!("{}", if i == n - 1 { "\n" } else { " " });
    }
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
