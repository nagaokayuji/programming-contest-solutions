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
        s:[f64;4],
    }
    let (mut sx, mut sy, mut gx, mut gy) = (s[0], s[1], s[2], s[3]);
    if sx > gx {
        swap(&mut sx, &mut gx);
        swap(&mut sy, &mut gy);
    }
    let mut ok = gx;
    let mut ng = sx;
    // while (ok - ng) > 1e-7 {
    for _ in 0..300 {
        let mid = (ok + ng) / 2f64;
        let dx = mid - sx;
        let dy = -sy;
        let refl = -dy;
        // if sx < gx {
        if refl * (gx - mid) <= gy * dx {
            ok = mid;
        } else {
            ng = mid;
        }
        // }
    }
    println!("{}", ok);
}
