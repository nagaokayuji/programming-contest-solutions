#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:i64,
       mut a:[i64;n],
    }
    let mut cumsum = vec![0; n + 1];
    for i in 0..n {
        a[i] -= 1;
    }
    for i in 0..n {
        cumsum[i + 1] = (cumsum[i] + a[i]) % k;
    }
    for mut x in cumsum.iter_mut() {
        *x %= k;
    }
    let mut mp = BTreeMap::new();
    let mut ans = 0;
    for (i, &x) in cumsum.iter().enumerate() {
        let i = i as i64;
        let l = mp.entry(x).or_insert(0);
        ans += *l;
        *l += 1;
        if i >= k - 1 {
            *mp.entry(cumsum[(i - k + 1) as usize]).or_insert(0) -= 1;
        }
    }
    println!("{}", ans);
}
