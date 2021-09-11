#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        k:i64,
        mut x:i64,mut y:i64,
    }
    if (k % 2 == 0) && ((x + y) % 2 == 1) {
        println!("{}", -1);
        return;
    }
    let mut xr = false;
    let mut yr = false;
    let mut xy = false;
    if x < 0 {
        x = -x;
        xr = true;
    }
    if y < 0 {
        y = -y;
        yr = true;
    }
    if x < y {
        swap(&mut x, &mut y);
        xy = true;
    }

    let mut anss = vec![];
    let mut n = max((x + y + k - 1) / k, 2);
    if (((x + y) % 2) != (n * k % 2)) {
        n += 1;
    }
    if x + y == k {
        anss.push((x, y));
    } else if (n == 3) && (x < k) {
        let mid = (k + x - y) / 2;
        anss.push((x, x - k));
        anss.push((x + mid, y - k + mid));
        anss.push((x, y));
    } else {
        let over = (n * k - x - y) / 2;
        for i in 1..=n {
            let d = i * k;
            if d <= y + over {
                anss.push((0, d));
            } else if d <= y + over + x {
                anss.push((d - y - over, y + over));
            } else {
                anss.push((x, y + (n - i) * k));
            }
        }
    }
    println!("{}", anss.len());
    for &x in anss.iter() {
        let (mut a, mut b) = x;
        if xy {
            swap(&mut a, &mut b);
        }
        if xr {
            a = -a;
        }
        if yr {
            b = -b;
        }
        println!("{} {}", a, b);
    }
}
