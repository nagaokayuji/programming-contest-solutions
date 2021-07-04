#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        h:usize,w:usize,m:usize,
        mut hw:[(Usize1,Usize1);m],
    }
    let mut st = BTreeSet::new();
    for &(h, w) in hw.iter() {
        st.insert((h, w));
    }
    //
    let mut xs = vec![0; MX];
    let mut ys = vec![0; MX];
    for &(h, w) in hw.iter() {
        xs[h] += 1;
        ys[w] += 1;
    }

    let mut st2 = st.clone();
    let mut ans = 0;
    // for hw in st {
    //     let (x,w) = hw;
    //     // x
    //     ans += xs[x];

    // }
    let mut xmax = 0;
    let mut ymax = 0;
    let mut sf = true;
    let mut xmaxs = vec![];
    let mut ymaxs = vec![];
    for i in 0..MX {
        if xs[i] > xs[xmax] {
            xmax = i;
            xmaxs = vec![i];
        }
        if ys[i] > ys[ymax] {
            ymax = i;
            ymaxs = vec![i];
        }
        if xs[i] == xs[xmax] {
            xmaxs.push(i);
        }
        if ys[i] == ys[ymax] {
            ymaxs.push(i);
        }
    }
    // dbg!(&xmaxs);
    // dbg!(&ymaxs);
    let mut ans = 0;
    for &x in xmaxs.iter() {
        for &y in ymaxs.iter() {
            if st.contains(&(x, y)) {
                ans = max(ans, xs[xmax] + ys[ymax] - 1);
            } else {
                ans = max(ans, xs[xmax] + ys[ymax]);
                println!("{}", ans);
                return;
            }
        }
    }
    println!("{}", ans);
}
const MX: usize = 1010101;
