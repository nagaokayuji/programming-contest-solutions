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
        r1:i64,c1:i64,
        r2:i64,c2:i64,
    }
    let src = (r1 + c1, r1 - c1);
    let trc = (r2 + c2, r2 - c2);
    // - (x,y) => (x+2k, y)
    // - (x,y) => (x, y+2k)
    // - xまたはyの差が3以下の地点
    if src == trc {
        println!("{}", 0);
        return;
    }
    if (src.0 - trc.0).abs() <= 3 && (src.1 - trc.1).abs() <= 3 {
        println!("{}", 1);
        return;
    }
    let mut flg = ((src.0 - trc.0) % 2 != 0) || ((src.1 - trc.1) % 2 != 0);
    let mut flg2 = (src.0 - trc.0).abs() <= 3 || (src.1 - trc.1).abs() <= 3;
    let mut flg3 = src.0 == trc.0 || src.1 == trc.1;

    if flg3 {
        println!("{}", 1 + if flg { 1 } else { 0 });
        return;
    }
    if flg2 {
        println!("{}", 2);
        return;
    }
    if (src.0 - trc.0).abs() <= 6 && (src.1 - trc.1).abs() <= 6 {
        println!("{}", 2);
        return;
    }
    if flg {
        println!("{}", 3);
    } else {
        println!("{}", 2);
    }
}
