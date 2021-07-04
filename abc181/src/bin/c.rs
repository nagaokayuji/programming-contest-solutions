#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
          n:usize,
    mut      xy:[(i64,i64);n],
      }

    xy.sort();
    for i in 0..n {
        let now = xy[i];

        for j in i + 1..n {
            let nx1 = xy[j];
            for k in j + 1..n {
                let nx2 = xy[k];

                let d1 = (nx1.0 - now.0, nx1.1 - now.1);
                let d2 = (nx2.0 - now.0, nx2.1 - now.1);
                if d1.1 * d2.0 == d1.0 * d2.1 {
                    yn(true);
                    return;
                }
            }
        }
    }
    yn(false);
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
