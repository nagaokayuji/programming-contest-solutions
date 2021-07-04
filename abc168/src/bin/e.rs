#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use proconio::{fastout, input};
use std::cmp::*;
use std::collections::*;
use std::mem::*;
#[fastout]
fn main() {
  input! {
    n:usize,
    mut ab:[(i64,i64);n],
  }
  let mut zeroc = 0;
  let mut mp = HashMap::<(i64, i64), (i64, i64)>::new();
  for (xx, yy) in ab {
    let mut x = xx;
    let mut y = yy;
    if x == 0 && y == 0 {
      zeroc += 1;
      continue;
    }
    if x == 0 {
      mp.entry((0, 0)).or_insert((0, 0)).0 += 1;
    } else if y == 0 {
      mp.entry((0, 0)).or_insert((0, 0)).1 += 1;
    } else {
      if y < 0 {
        x = -x;
        y = -y;
      }
      let g = gcd(x.abs(), y.abs());
      x /= g;
      y /= g;
      if x < 0 {
        mp.entry((y, -x)).or_insert((0, 0)).0 += 1;
      } else {
        mp.entry((x, y)).or_insert((0, 0)).1 += 1;
      }
    }
  }
  let mut ans = 1i64;
  for (key, value) in mp {
    let a = mod_pow(2, value.0, MOD) + mod_pow(2, value.1, MOD) - 1;
    ans = (ans * a) % MOD;
  }
  ans = (ans + zeroc + MOD - 1) % MOD;
  println!("{}", ans);
}
pub fn gcd(a: i64, b: i64) -> i64 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}
static MOD: i64 = 1000000007;
pub fn mod_pow(x: i64, n: i64, m: i64) -> i64 {
  let mut res = 1;
  let mut x = x % m;
  let mut n = n;
  while n > 0 {
    if n & 1 == 1 {
      res = (res * x) % m;
    }
    x = (x * x) % m;
    n >>= 1;
  }
  res
}
