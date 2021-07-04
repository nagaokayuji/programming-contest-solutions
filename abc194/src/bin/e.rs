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
        n:usize,m:usize,
        mut a:[usize;n],
    }

    let mut ans = 1e7 as usize;
    let mut mex = 0; // default

    let mut l = 0;
    let mut r = 0;
    let mut mp = vec![0; 1511111];
    let mut st = BTreeSet::new();
    for x in 0..1500011 {
        st.insert(x as usize);
    }
    while l < n - m + 1 {
        while r < n && r - l < m {
            mp[a[r]] += 1;
            st.remove(&a[r]);
            mex = *st.iter().next().unwrap();
            r += 1;
        }
        if r - l != m {
            break;
        }
        ans = min(ans, mex);
        mp[a[l]] -= 1;
        if mp[a[l]] == 0 {
            // mex = a[l];
            st.insert(a[l]);
        }
        // ans = min(ans, mex);
        l += 1;
    }
    println!("{}", ans);
}
const INF: i64 = 1 << 60;
