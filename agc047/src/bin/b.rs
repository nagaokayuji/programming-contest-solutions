#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
/// Rolling hash algorithm
pub struct RollingHash {
    hash_pow_list: Vec<(u64, Vec<(u64, u64)>)>,
}
impl RollingHash {
    pub fn new(s: &[u64]) -> RollingHash {
        RollingHash::with_base_mod_pairs(s, &[(1009, 1_000_000_007), (9973, 999_999_937)])
    }
    pub fn with_base_mod_pairs(s: &[u64], base_mod_pairs: &[(u64, u64)]) -> RollingHash {
        let hp_list = base_mod_pairs
            .iter()
            .map(|&(base, m)| {
                let mut hp = Vec::with_capacity(s.len() + 1);
                hp.push((0, 1));
                for (i, &x) in s.iter().enumerate() {
                    let (h, p) = hp[i];
                    hp.push(((h + x) * base % m, p * base % m));
                }
                (m, hp)
            })
            .collect();
        RollingHash {
            hash_pow_list: hp_list,
        }
    }
    pub fn get(&self, l: usize, r: usize) -> u64 {
        self.hash_pow_list
            .iter()
            .map(|&(m, ref hp)| (hp[r].0 + m - hp[l].0 * hp[r - l].1 % m) % m)
            .fold(0, |a, b| a ^ b)
    }
    pub fn len(&self) -> usize {
        self.hash_pow_list
            .first()
            .map(|v| v.1.len() - 1)
            .unwrap_or(0)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
#[fastout]
fn main() {
    input! {//
        n:usize,
       mut  s:[Bytes;n],
    }
    let mut ss = vec![];
    for x in s.iter() {
        for i in (0..x.len()).rev() {
            ss.push(x[i] as u64);
        }
    }
}
