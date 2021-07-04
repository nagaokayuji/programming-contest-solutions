#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,
    }
    let mut anss = vec![0; n + 1];
    for x in 1..=n {
        if x * x >= n {
            continue;
        }
        for y in 1..=n {
            if x * x + y * y + x * y > n {
                break;
            }
            // z*z + y*z + z*x = n
            // let z = n-x*x-y*y-x*y
            for z in 1..n {
                // if z * z + y * z + z * x + x * x + y * y + x * y == n {
                //     ans += 1;
                // }
                let s = x * x + y * y + z * z + x * y + x * z + y * z;
                if s > n {
                    break;
                } else {
                    anss[s as usize] += 1;
                }
            }
        }
    }
    for i in 1..=n {
        println!("{}", anss[i]);
    }
}
