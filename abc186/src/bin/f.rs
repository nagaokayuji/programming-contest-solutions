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
        h:usize,w:usize,m:usize,
        mut xy:[(Usize1,Usize1);m],
    }
    if m == 0 {
        println!("{}", h * w);
        return;
    }
    if m == 1 {
        println!("{}", h * w - 1);
        return;
    }
    let mut xs = vec![BTreeSet::new()];
    let mut ys = vec![BTreeSet::new()];
}
