#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        n:usize,
        mut a: [i64;n],
        mut b: [i64;n],
    }
    a.sort();
    b.sort();

    let mut koho = HashSet::new();
    for &x in b.iter() {
        koho.insert(a[0] ^ x);
    }
    let mut ans = vec![];
    for &k in koho.iter() {
        let mut mv = a.iter().map(|&x| x ^ k).collect::<Vec<_>>();
        mv.sort();
        if b.iter().zip(mv.iter()).all(|(&x, &y)| x == y) {
            ans.push(k);
        }
    }
    ans.sort();
    println!("{}", ans.len());
    for &x in ans.iter() {
        println!("{}", x);
    }
}
