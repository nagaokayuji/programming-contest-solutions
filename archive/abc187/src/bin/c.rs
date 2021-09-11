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
        s:[Bytes;n],
    }
    let mut t = BTreeSet::new();
    let mut f = BTreeSet::new();

    for x in s.iter() {
        if x[0] == b'!' {
            t.insert(&x[1..]);
        } else {
            f.insert(&x[0..]);
        }
    }
    for x in t {
        if f.contains(&x) {
            for &xx in x.iter() {
                print!("{}", xx as char);
            }
            return;
        }
    }
    println!("satisfiable");
}
