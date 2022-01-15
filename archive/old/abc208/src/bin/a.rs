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
       mut ab:[(i64,i64);n],
    }
    let mut ans = 1e6 as i64;
    ab.sort();
    let mut ba = ab.iter().map(|&x| (x.1, x.0)).collect::<Vec<_>>();
    ba.sort();

    let amin = ab[0].0;
    let bmin = ba[0].0;

    if ab.iter().filter(|&x| x.0 == amin || x.1 == bmin).count() > 1 {
        // dbg!("hoh");
        println!("{}", max(amin, bmin));
        return;
    }

    ans = min(
        max(ab[0].0, ba[1].0),
        min(max(ab[1].0, ba[0].0), ab[0].0 + ab[0].1),
    );
    println!("{}", ans);
}
