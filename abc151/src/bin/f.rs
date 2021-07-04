#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
const eps: f64 = 1e-10;
fn main() {
    input! {//
        n:usize,
        mut xy:[(f64,f64);n],
    }
    // let mut gr = xy
    //     .iter()
    //     .fold((0f64, 0f64), |sum, &x| (sum.0 + x.0, sum.1 + x.1));
    // gr = (gr.0 / n as f64, gr.1 / n as f64);
    let mut now = (0f64, 0f64);
    let mut ans = 12345f64;
    let mut mul = 2000f64;
    for _ in 0..40000 {
        // 最も遠い点への距離
        let mut tmp = 0f64;
        let mut longest = (0f64, 0f64);
        for &(x, y) in xy.iter() {
            let dis = ((x - now.0).powi(2) + (y - now.1).powi(2)).sqrt();
            if tmp < dis {
                tmp = dis;
                longest = (x, y);
            }
        }
        if ans > tmp {
            ans = tmp;
        }
        // step
        let (dx, dy) = (longest.0 - now.0, longest.1 - now.1);
        let ln = (dx.powi(2) + dy.powi(2)).sqrt();

        now = (now.0 + dx * mul / ln, now.1 + dy * mul / ln);
        mul = mul * 0.995f64;
    }
    println!("{}", ans);
}
