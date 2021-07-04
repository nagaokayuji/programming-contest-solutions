#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
/*
    ■
*/
#[fastout]
fn main() {
    input! {//
        x:i128,y:i128,
    }
    if x >= y {
        println!("{}", x - y);
        return;
    }
    let mut cnt = 0;
    let mut g = G {
        x: x,
        y: y,
        dp: HashMap::new(),
    };
    println!("{}", g.solve(y));
}
struct G {
    x: i128,
    y: i128,
    dp: HashMap<i128, i128>,
}
impl G {
    fn solve(&mut self, k: i128) -> i128 {
        if self.x == k {
            return 0;
        }
        if k <= 1 {
            return (self.x - k).abs();
        }
        let dp = *self.dp.entry(k).or_insert(0);
        if dp > 0 {
            return dp;
        }
        let mut ret = (self.x - k).abs();
        if k % 2 == 1 {
            chmin!(
                ret,
                self.solve((k + 1) / 2) + 2,
                self.solve((k - 1) / 2) + 2
            );
        } else {
            chmin!(ret, self.solve(k / 2) + 1);
        }

        *self.dp.entry(k).or_insert(0) = ret;
        return ret;
    }
}
