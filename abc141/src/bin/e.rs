#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        s:Chars,
    }
    let mut su128 = s.iter().map(|&c| c as u128).collect::<Vec<_>>();
    let mut rh = RollingHash::new(&su128);
    let mut ok = 0;
    let mut ng = n / 2 + 2;
    while ng - ok > 1 {
        // length
        let isOk = |x: usize| {
            let mut mp = BTreeMap::new();
            for i in 0..=n {
                if i + x > n {
                    break;
                }
                let now = rh.get(i, i + x);
                if let Some(y) = mp.get(&now) {
                    // . . . . y . . i . .
                    if i - y >= x {
                        return true;
                    }
                } else {
                    mp.insert(now, i);
                }
            }
            return false;
        };
        let mid = (ok + ng) / 2;
        if isOk(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
/// Rolling hash algorithm
/// ```
/// // example
/// s:Vec<char> = vec!['a','b','c','d'];
/// let mut su128 = s.iter().map(|&c| c as u128).collect::<Vec<_>>();
/// let mut rh = RollingHash::new(&su128);
/// rh.get(l,r) = rh.get(ll,rr)
/// ```
pub struct RollingHash {
    hash_pow_list: Vec<(u128, Vec<(u128, u128)>)>,
}
impl RollingHash {
    pub fn new(s: &[u128]) -> RollingHash {
        RollingHash::with_base_mod_pairs(s, &[(809, 1u128 << 61 - 1), (997, 1u128 << 31 - 1)])
    }
    pub fn with_base_mod_pairs(s: &[u128], base_mod_pairs: &[(u128, u128)]) -> RollingHash {
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
    /// get hash of [l,r)
    pub fn get(&self, l: usize, r: usize) -> u128 {
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
