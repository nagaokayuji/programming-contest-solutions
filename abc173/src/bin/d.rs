#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
     n:usize,
    mut a: [i64;n],
      }
    a.sort();

    a.reverse();
    // dbg!(&a);
    //  for i in 0..n {
    //      if i==0 {
    //          // a[0]到着

    //      }
    //      else {
    //          a[i]
    //      }
    //  }
    let mut ans = 0;
    ans += a[0];
    for i in 2..n {
        ans += a[i / 2];
    }
    println!("{}", ans);
}
