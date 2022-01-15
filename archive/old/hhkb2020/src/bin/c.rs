#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut p:[i64;n],
    }
    let mut st = BTreeSet::new();
    for i in 0..=n {
        st.insert(i as i64);
    }
    for &x in p.iter() {
        st.insert(x);
    }
    let mut prv = 0;
    let mut prv = p.iter().max().unwrap();

    for &x in p.iter() {
        st.remove(&x);
        if st.len() > 0 {
            println!("{}", *st.iter().next().unwrap());
        } else {
            println!("{}", prv + 1);
        }
    }
    // let mut pp = p.clone();
    // pp.sort();
    // let mut x1 = pp[0];
    // let mut x2 = INF;
    // for &x in pp.iter() {
    //     if x2 > x && x != x1 {
    //         x2 = x;
    //         break;
    //     }
    // }

    // for &x in p.iter() {
    //     if x == x1 {
    //         println!("{}", x2);
    //     } else {
    //         println!("{}", x1);
    //     }
    // }
}
const INF: i64 = 1 << 60;
