#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {
    n:usize,
    s:Chars,
     }

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut ast = BTreeSet::new();
    let mut bst = BTreeSet::new();
    let mut cst = BTreeSet::new();
    for (i, &x) in s.iter().enumerate() {
        if x == 'R' {
            a.push(i);
            ast.insert(i);
        } else if x == 'G' {
            b.push(i);
            bst.insert(i);
        } else {
            c.push(i);
            cst.insert(i);
        }
    }

    let al = a.len();
    let bl = b.len();
    let cl = c.len();
    let mut ans = al * bl * cl;
    for i in 0..al {
        for j in 0..bl {
            let mut dif = 0;
            if a[i] < b[j] {
                dif = b[j] - a[i];
                if cst.contains(&(b[j] + dif)) {
                    ans -= 1;
                }
            } else {
                dif = a[i] - b[j];
                if cst.contains(&(a[i] + dif)) {
                    ans -= 1;
                }
            }
        }
    }
    for i in 0..bl {
        for j in 0..cl {
            if b[i] < c[j] {
                let dif = c[j] - b[i];
                if ast.contains(&(c[j] + dif)) {
                    ans -= 1;
                }
            } else {
                let dif = b[i] - c[j];
                if ast.contains(&(b[i] + dif)) {
                    ans -= 1;
                }
            }
        }
    }
    for i in 0..al {
        for j in 0..cl {
            if a[i] < c[j] {
                let dif = c[j] - a[i];
                if bst.contains(&(c[j] + dif)) {
                    ans -= 1;
                }
            } else {
                let dif = a[i] - c[j];
                if bst.contains(&(a[i] + dif)) {
                    ans -= 1;
                }
            }
        }
    }
    println!("{}", ans);
}
/// (gcd, x, y)
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extgcd(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}
pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
/// x ^ n % m
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
pub fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extgcd(a as i64, m as i64);
    ((m as i64 + x) as i64 % m) % m
}
pub fn fact_table(len: usize, m: i64) -> Vec<i64> {
    let mut res = vec![1; len + 1];
    for i in 1..len + 1 {
        res[i] = (i as i64 * res[i - 1]) % m;
    }
    res
}
/// Factorial and Inverse factorial table
pub fn fact_inv_table(size: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![1; size];
    let mut fact_inv = vec![1; size];
    for i in 2..size {
        fact[i] = fact[i - 1] * i as i64 % m;
        fact_inv[i] = m - ((m / i as i64) * fact_inv[(m % i as i64) as usize] % m);
    }
    for i in 1..size {
        fact_inv[i] = fact_inv[i - 1] * fact_inv[i] % m;
    }
    (fact, fact_inv)
}
/// (a mod p, e when n! = a p\^e)
pub fn mod_fact(n: i64, p: i64, fact: &[i64]) -> (i64, i64) {
    if n == 0 {
        (1, 0)
    } else {
        let (a, b) = mod_fact(n / p, p, fact);
        let pow = b + n / p;
        if n / p % 2 != 0 {
            (a * (p - fact[(n % p) as usize]) % p, pow)
        } else {
            (a * fact[(n % p) as usize] % p, pow)
        }
    }
}
/// C(n, k) % p
pub fn mod_comb(n: i64, k: i64, p: i64, fact: &[i64]) -> i64 {
    if n < k {
        0
    } else {
        let (a1, e1) = mod_fact(n, p, fact);
        let (a2, e2) = mod_fact(k, p, fact);
        let (a3, e3) = mod_fact(n - k, p, fact);
        if e1 > e2 + e3 {
            0
        } else {
            a1 * mod_inverse(a2 * a3 % p, p) % p
        }
    }
}
/// H(n, k) % p
pub fn mod_comb_repetition(n: i64, k: i64, p: i64, fact: &[i64]) -> i64 {
    mod_comb(n - 1 + k, n - 1, p, fact)
}
