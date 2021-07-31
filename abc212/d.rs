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
        q:usize,
    }
    let mut mp = BTreeMap::new();
    let mut sum = 0i64;
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {x:i64}
                *mp.entry(x - sum).or_insert(0) += 1;
            }
            2 => {
                input! {x:i64}
                sum += x;
            }
            _ => {
                let (&k, &v) = mp.iter().next().unwrap();
                println!("{}", k + sum);
                *mp.entry(k).or_default() -= 1;
                if v == 1 {
                    mp.remove(&k);
                }
            }
        }
    }
}
