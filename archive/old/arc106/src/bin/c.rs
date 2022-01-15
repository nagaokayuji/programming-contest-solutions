#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,m:i64,
    }
    if n == 1 {
        if m == 0 {
            println!("{} {}", 3, 5);
            return;
        } else {
            println!("{}", -1);
            return;
        }
    }
    if n == 2 {
        if m == 0 {
            println!("{} {}", 1, 3);
            println!("{} {}", 4, 6);
            return;
        } else if m == 1 {
            println!("{}", -1);
            return;
        }
        println!("{}", -1);
        return;
    }
    if m < 0 {
        println!("{}", -1);
        return;
    }
    if m == n as i64 {
        println!("{}", -1);
        return;
    }
    if m + 1 == n as i64 {
        println!("{}", -1);
        return;
    }

    let mut lastr = 3;
    let mut anss = vec![];
    for l in 0..n - 1 {
        let li = lastr + 2;
        let ri = li + 1;
        // println!("{} {}", li, ri);
        anss.push((li, ri));
        lastr = ri;
    }
    if m == 0 {
        anss.push((lastr + 2, lastr + 3));
        printans(&anss);
        return;
    }

    // if m + 3 < n as i64 {
    //     let tmp = anss[n as usize - m as usize - 3].1;
    //     // dbg!(tmp);
    //     anss.push((tmp + 1, 10000000));
    //     printans(&anss);
    //     return;
    //     // dbg!(&anss);
    // }
    let tmp = anss[m as usize];
    anss.push((1, tmp.1 + 1));
    printans(&anss);

    // println!("{}", -1);
    // return;
}

const max: i64 = 100000000;

fn printans(ans: &Vec<(usize, usize)>) {
    for &(l, r) in ans.iter() {
        println!("{} {}", l, r);
    }
}
