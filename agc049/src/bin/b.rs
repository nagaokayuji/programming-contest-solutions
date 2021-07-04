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
        n:usize,
        mut s:Chars,
        mut t:Chars,
    }
    let soc = s.iter().filter(|&&x| x == '1').count();
    let toc = t.iter().filter(|&&x| x == '1').count();
    // // s は '1' が減っていく
    // if soc < toc {
    //     println!("{}", -1);
    //     return;
    // }
    // // 減るときは二個
    // if soc % 2 != toc % 2 {
    //     println!("{}", -1);
    //     return;
    // }

    // s.reverse();
    // t.reverse();
    // dbg!(&s);
    // dbg!(&t);
    let mut cnt = 0usize;
    let mut del1 = 0usize;
    let mut need = vec![];
    let mut del = vec![];
    let mut need1 = 0usize;
    for i in 0..n {
        if s[i] == '1' && del.len() > 0 {
            let target = del.pop().unwrap();
            cnt += i - target;
            s[i] = '0';
            s[target] = '0';
        }
        if s[i] == '1' && need.len() > 0 {
            let target = need.pop().unwrap();
            cnt += i - target;
            s[target] = '1';
            s[i] = '0';
        }
        if s[i] == '1' && t[i] == '0' {
            del.push(i);
            // del1 = i;
        }
        if s[i] == '0' && t[i] == '1' {
            need.push(i);
            // need1 = i;
        }
        // if t[i] == '1' {
        //     need.push(i);
        // }
    }
    for i in 0..n {
        if s[i] != t[i] {
            println!("{}", -1);
            return;
        }
    }
    println!("{}", cnt);
}
