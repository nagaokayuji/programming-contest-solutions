#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
        mut x:Chars,
    }
    x.reverse();
    let mut basecount = 0i64;
    for &c in x.iter() {
        if c == '1' {
            basecount += 1;
        }
    }
    let mut dp = vec![0; MX as usize];
    for i in 0..MX as usize {
        if i == 0 {
            dp[0] = 0;
        }
        if i == 1 {
            dp[1] = 1;
        } else {
            let mut tmp = i;
            let mut cnt = 0;
            while tmp > 0 {
                // dbg!(tmp, tmp.count_ones());
                tmp %= tmp.count_ones() as usize;
                cnt += 1;
            }
            dp[i] = cnt;
        }
    }
    if n == 1 {
        if x[0] == '1' {
            println!("{}", 0);
            return;
        } else {
            println!("{}", 1);
            return;
        }
    }

    // dbg!(&dp);

    let mut fbasep = 0;
    let mut fbasen = 0;
    for i in 0..n {
        if basecount == 0 {
            break;
        }
        if x[i] == '1' {
            fbasep += mod_pow(2, i as i64, basecount + 1);
            if basecount > 1 {
                fbasen += mod_pow(2, i as i64, basecount - 1);
            }
        }
        // fbase %= basecount;
    }
    // println!("{}", fbase);

    for i in (0..n).rev() {
        let mut bc = basecount;
        if basecount == 0 {
            println!("{}", 1);
            continue;
        }
        let mut pn = 1i64;
        if x[i] == '1' {
            pn = -1;
        } else {
            pn = 1;
        }
        bc += pn;
        if pn == 1 {
            let mut b = fbasep + mod_pow(2, i as i64, bc);
            b %= bc;
            println!("{}", dp[b as usize] + 1);
        } else {
            if bc == 0 {
                println!("{}", 0);
                continue;
            }
            if bc == 1 {
                println!("{}", 1);
                continue;
            }
            let mut b = fbasen + bc - mod_pow(2, i as i64, bc);
            b %= bc;
            println!("{}", dp[b as usize] + 1);
        }
    }
    // for i in 0..10 {
    //     dbg!(i, dp[i]);
    // }
}
const MX: i64 = 1000100;
const INF: i64 = 1 << 60;
/// x ^ n % m
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
    let mut res = 1;
    let mut x = x % m;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = (res * x) % m;
        }
        x = (x * x) % m;
        n >>= 1;
    }
    res
}
