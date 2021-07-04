#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
       n:usize,
        mut s:[Chars;n],
    }
    let mut ans = 0i64;
    let mut mp = HashMap::new();
    for mut ss in s.iter() {
        let mut k = ss.clone();
        k.sort();
        let c = mp
            .entry(k.iter().collect::<String>().clone())
            .or_insert(0i64);
        ans += *c;
        *c += 1i64;
    }
    println!("{}", ans);
}
