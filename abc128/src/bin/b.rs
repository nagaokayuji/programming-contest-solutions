#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
     mut   sp:[(String,i64);n],
    }
    let mut ss = vec![];
    let mut mp = BTreeMap::new();
    for (ind, (city, point)) in sp.iter().cloned().enumerate() {
        ss.push((city.clone(), point, ind + 1));
        mp.entry(city.clone())
            .or_insert(Vec::new())
            .push((point, ind + 1));
    }
    // dbg!(&ss);
    // // ss.sort();
    // // ss.reverse();
    // for (city, point, ind) in ss.iter() {
    //     println!("{}", ind);
    // }
    for (key, value) in mp {
        let mut v = value.clone();
        v.sort_by_key(|&x| -x.0);
        for &x in v.iter() {
            println!("{}", x.1);
        }
    }
}
