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
    }
    let mut allgcd = gcd_list(&a);
    let mut st = BTreeSet::new();
    let mut all = true;
    for &x in a.iter() {
        if x != 1 {
            let mut pf = prime_factor(x as i64);
            for (k, _) in pf {
                let kk = k;
                if st.contains(&k) {
                    all = false;
                    break;
                } else {
                    st.insert(kk);
                }
            }
            if !all {
                break;
            }
        }
    }
    if all {
        println!("{}", "pairwise coprime");
    } else {
        if allgcd == 1 {
            println!("{}", "setwise coprime");
        } else {
            println!("{}", "not coprime");
        }
    }
}
fn prime_factor(mut n: i64) -> std::collections::HashMap<i64, i64> {
    let mut res: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let v = match res.get(&i) {
                Some(v) => *v + 1,
                None => 1,
            };
            res.insert(i, v);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn gcd_list(list: &[usize]) -> usize {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}
