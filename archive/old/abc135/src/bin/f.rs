#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
macro_rules! chmax {
    ($ base : expr ,$ cmp : expr ) => {
        $base = max($base, $cmp);
    };
}

#[fastout]
fn main() {
    input! {//
       mut s:Chars,
        mut t:Chars,
    }
    let s = con(&s);
    let mut ss = t.clone();
    ss.push('$');
    while ss.len() <= 2 * t.len() + t.len() + 1 {
        for &c in s.iter() {
            ss.push(c);
        }
    }
    for &c in s.iter() {
        ss.push(c);
    }
    let mut z = zalgo(&ss);
    let mut dp = vec![0; 3030300];
    let mut ans = 0;
    let mut i = ss.len() - 1;
    while ss[i] != '$' {
        if z[i] == t.len() {
            dp[i] = dp[i + t.len()] + 1;
        }
        chmax!(ans, dp[i]);
        i -= 1;
    }
    if ans > 0 && (t.len() % s.len() == 0) {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}

pub fn zalgo(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut a = vec![0usize; n];
    let mut from = 0;
    let mut last = 0;
    a[0] = n;
    for i in 1..n {
        let mut same = a[i];
        if from != 0 {
            same = std::cmp::min(a[i - from], if last > i { last - i } else { 0 });
        }
        while i + same < n && s[same] == s[same + i] {
            same += 1;
        }
        if last < i + same {
            last = i + same;
            from = i;
        }
        a[i] = same;
    }
    return a;
}

fn kmp(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut i = 0;
    let mut j = -1;
    let mut a = vec![!0; n + 1];
    while i < n {
        while (!j != 0 && s[i] != s[j as usize]) {
            j = a[j as usize] as i64;
        }
        a[i + 1] = (j + 1) as usize;
        i += 1;
        j += 1;
    }
    a
}
fn con(s: &Vec<char>) -> &[char] {
    let kmp = kmp(s);
    let c = s.len() - kmp[s.len()];
    if s.len() % c == 0 {
        &s[0..c]
    } else {
        &s[..]
    }
}
