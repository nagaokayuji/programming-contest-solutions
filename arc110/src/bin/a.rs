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
    }
    let mut m = vec![];
    for i in 2..=n {
        m.push(i as i64);
    }
    let k = lcm_list(&m);
    println!("{}", k + 1);
}
pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm_list(list: &[i64]) -> i64 {
    list.iter().fold(list[0], |a, &b| lcm(a, b))
}

pub(crate) fn inv_gcd(a: i64, b: i64) -> (i64, i64) {
    let a = safe_mod(a, b);
    if a == 0 {
        return (b, 0);
    }
    let mut s = b;
    let mut t = a;
    let mut m0 = 0;
    let mut m1 = 1;
    while t != 0 {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u;
        swap(&mut s, &mut t);
        swap(&mut m0, &mut m1);
    }
    if m0 < 0 {
        m0 += b / s;
    }
    (s, m0)
}
fn safe_mod(mut x: i64, m: i64) -> i64 {
    x %= m;
    if x < 0 {
        x += m;
    }
    x
}
pub fn crt(r: &[i64], m: &[i64]) -> (i64, i64) {
    assert_eq!(r.len(), m.len());
    let (mut r0, mut m0) = (0, 1);
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        assert!(1 < mi);
        ri = safe_mod(ri, mi);
        if m0 < mi {
            swap(&mut r0, &mut ri);
            swap(&mut m0, &mut mi);
        }
        if m0 % mi == 0 {
            if r0 % mi != ri {
                return (0, 0);
            }
            continue;
        }
        let (g, im) = inv_gcd(m0, mi);
        let u1 = mi / g;
        if (ri - r0) % g != 0 {
            return (0, 0);
        }
        let x = (ri - r0) / g % u1 * im % u1;
        r0 += x * m0;
        m0 *= u1;
        if r0 < 0 {
            r0 += m0
        };
    }
    (r0, m0)
}
