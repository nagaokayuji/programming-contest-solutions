#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut ab:[(i64,i64);n],
    }
    ab.sort();
    let mut kakutei = vec![];
    let mut fumei = vec![];
    let mut ans = true;
    let mut found = vec![];
    let mut used = BTreeSet::new();
    let mut akakutei = vec![];
    let mut bkakutei = vec![];
    let mut bothfumei = vec![];
    let mut wild = 0;

    for &(a, b) in ab.iter() {
        if b != -1 {
            if used.contains(&b) {
                ans = false;
            }
            used.insert(b);
        }
        if a != -1 {
            if used.contains(&a) {
                ans = false;
            }
            used.insert(a);
        }
        if b == 1 {
            ans = false;
        }
        if a != -1 && b != -1 {
            if a >= b {
                ans = false;
            }
            if !found.contains(&a) && !found.contains(&b) {
                found.push(a);
                found.push(b);
                kakutei.push((a, b));
            } else {
                ans = false;
            }
        } else {
            fumei.push((a, b));
        }
        if a != -1 && b == -1 {
            akakutei.push((a, b));
        }
        if a == -1 && b != -1 {
            bkakutei.push((a, b));
        }
        if a == -1 && b == -1 {
            bothfumei.push((a, b));
            wild += 1;
        }
    }
    if !ans {
        yn(false);
        return;
    }
    let mut t = vec![None; 2 * n];
    for (i, &(a, b)) in ab.iter().enumerate() {
        if a > 0 {
            if t[a as usize - 1].is_some() {
                ans = false;
            }
            t[a as usize - 1] = Some((i, false));
        }
        if b > 0 {
            if t[b as usize - 1].is_some() {
                ans = false;
            }
            t[b as usize - 1] = Some((i, true));
        }
    }
    let mut dp = vec![vec![false; n + 2]; n + 2];
    for k in 1..=n {
        for i in 0..=n - k {
            let mut ok = true;
            for j in 0..k {
                // both
                if let (Some((x, b1)), Some((y, b2))) = (t[2 * i + j], t[2 * i + k + j]) {
                    ok &= x == y && b1 == false && b2 == true
                }
                // st
                if let Some((_, b)) = t[2 * i + j] {
                    ok &= !b;
                }
                // ed
                if let Some((_, b)) = t[2 * i + k + j] {
                    ok &= b;
                }
            }
            if ok {
                dp[k][i] = true;
            }
            for j in 1..k {
                if dp[j][i] && dp[k - j][i + j] {
                    dp[k][i] = true;
                }
            }
        }
    }
    ans &= dp[n][0];
    yn(ans);

    // akakutei.sort();
    // bkakutei.sort_by_key(|&x| x.1);
    // let mut r = vec![0; 2 * n + 1];
    // for &(a, b) in akakutei.iter() {
    //     r[a as usize] = -1;
    // }
    // for &(a, b) in bkakutei.iter() {
    //     r[b as usize] = -2;
    // }
    // for &(a, b) in kakutei.iter() {
    //     r[a as usize] = b;
    //     r[b as usize] = a;
    // }
    // let mut kakuteis = BTreeSet::new();
    // let mut akakuteis = BTreeSet::new();
    // let mut bkakuteis = BTreeSet::new();
    // for &x in kakutei.iter() {
    //     kakuteis.insert(x);
    // }
    // for &x in akakutei.iter() {
    //     akakuteis.insert(x);
    // }
    // for &x in bkakutei.iter() {
    //     bkakuteis.insert(x);
    // }
    // for i in 1..=2 * n {
    //     if r[i] < 0 {
    //         if r[i] == -1 {
    //             for rank in 1..=2 * n {
    //                 if i + rank > 2 * n {
    //                     break;
    //                 }
    //                 if r[i + rank] != 0 {
    //                     continue;
    //                 }
    //                 if i + rank + 1 <= 2 * n {
    //                     let k = i + rank + 1;
    //                     if r[k] > 0 && r[k] < k as i64 {
    //                         continue;
    //                     }
    //                     if r[k] == -2 {
    //                         continue;
    //                     }
    //                     r[i] = (k - 1) as i64;
    //                 }
    //             }
    //             if r[i] != -1 {
    //                 kakuteis.insert((i as i64, r[i]));
    //                 akakuteis.remove(&(i as i64, -1));
    //             }
    //         }
    //     }
    // }
    // dbg!(&kakuteis);
    // // dbg!(&akakuteis);
    // // dbg!(&bkakuteis);
    // // dbg!(ans);

    // if !ans {
    //     yn(false);
    //     return;
    // }
    // for _ in 0..150 {
    //     if !ans {
    //         break;
    //     }
    //     let mut nx = kakuteis.clone();
    //     for &(a, b) in kakuteis.iter() {
    //         let mut del = vec![];
    //         let mut add = vec![];
    //         let rank = b - a;
    //         for &(aa, bb) in kakuteis.iter() {
    //             if (a + 1..b).contains(&aa) {
    //                 if bb - aa != rank {
    //                     ans = false;
    //                 }
    //             }
    //             if (a + 1..b).contains(&bb) {
    //                 if bb - aa != rank {
    //                     ans = false;
    //                 }
    //             }
    //         }
    //         for &(aa, bb) in akakuteis.iter() {
    //             if (a + 1..b).contains(&aa) {
    //                 let bkoho = aa + rank;
    //                 if used.contains(&bkoho) {
    //                     ans = false;
    //                 }
    //                 used.insert(bkoho);
    //                 add.push((aa, bkoho));
    //                 del.push((aa, bb));
    //             }
    //         }
    //         // for &x in del.iter() {
    //         //     add.push(x);
    //         // }
    //         for &x in del.iter() {
    //             akakuteis.remove(&x);
    //         }
    //         del.clear();
    //         for &(aa, bb) in bkakuteis.iter() {
    //             if (a + 1..b).contains(&bb) {
    //                 let akoho = bb - rank;
    //                 if used.contains(&akoho) {
    //                     ans = false;
    //                     break;
    //                 }
    //                 used.insert(akoho);
    //                 add.push((akoho, bb));
    //                 del.push((aa, bb));
    //             }
    //         }
    //         for &x in del.iter() {
    //             bkakuteis.remove(&x);
    //         }
    //         for &x in add.iter() {
    //             nx.insert(x);
    //         }
    //         if !ans {
    //             break;
    //         }
    //     }
    //     kakuteis = nx.clone();
    // }
    // for &(a, b) in kakuteis.iter() {
    //     let rank = b - a;
    //     for i in a + 1..b {
    //         if !used.contains(&i) {
    //             used.insert(i);
    //             let target = rank + i;
    //             if target > 2 * n as i64 {
    //                 ans = false;
    //             }
    //             if used.contains(&target) {
    //                 ans = false;
    //             }
    //             used.insert(target);
    //         }
    //     }
    // }
    // yn(ans);
}
pub fn yn(ans: bool) {
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
