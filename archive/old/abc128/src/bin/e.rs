#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*, ops::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,q:usize,
        mut stx:[(i64,i64,i64);n],
        d:[i64;q],
    }
    let mut rng = vec![];
    for (s, t, x) in stx {
        rng.push((s - x, 0, x));
        rng.push((t - x, -1, x));
    }
    for &d in d.iter() {
        rng.push((d, 1, INF));
    }
    // 時間でソート
    rng.sort();
    let mut st = BTreeSet::new();
    for &(x, typ, val) in rng.iter() {
        if typ == 0 {
            st.insert(val);
        } else if typ == -1 {
            st.remove(&val);
        } else {
            // 工事中
            println!(
                "{}",
                if let Some(&x) = st.iter().next() {
                    x
                } else {
                    -1
                }
            );
        }
    }
}
const INF: i64 = 1 << 60;
