#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
fn main() {
    input! {//
        a:i64,b:i64,
    }

    let mut ad = divisor(a as usize);
    let mut bd = divisor(b as usize);

    let ah = prime_factor(a);
    let bh = prime_factor(b);

    let mut ans = 1;
    let mut aset = BTreeSet::new();
    for (k, _) in ah {
        aset.insert(k);
    }
    let mut bset = BTreeSet::new();
    for (k, _) in bh {
        bset.insert(k);
    }

    for x in aset.iter() {
        if bset.contains(&x) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
pub fn divisor(n: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in (1..).take_while(|x| x * x <= n) {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
    }
    res.sort();
    res
}

fn prime_factor(mut n: i64) -> std::collections::HashMap<i64, i64> {
    let mut res: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let v = match res.get(&i) {
                Some(v) => *v + 1,
                None => 1,
            };
            res.insert(i, v);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}
