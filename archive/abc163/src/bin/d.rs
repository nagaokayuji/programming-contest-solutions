#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
        n:usize,k:usize,
    }
    let mut ans = 0;
    let mut sms = vec![0; n + 2];
    for i in 1..=(n + 1) {
        sms[i] = sms[i - 1] + i - 1;
    }
    // dbg!(&sms);
    for c in k..=(n + 1) {
        let k = sms[n + 1] - sms[n + 1 - c];
        let l = sms[c];
        // dbg!(c, k, l);
        ans += k - l + 1;
        ans %= MOD;
    }
    println!("{}", ans);
}
const MOD: usize = 1000000007;
