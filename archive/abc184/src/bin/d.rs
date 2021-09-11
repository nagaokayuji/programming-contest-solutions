#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
    括弧の対応を間違えていた
*/
#[fastout]
fn main() {
    input! {//
        a:usize,b:usize,c:usize,
    }
    let mut g = G {
        mem: vec![vec![vec![-1f64; 100]; 100]; 100],
    };
    println!("{}", g.rec(a, b, c));
}
struct G {
    mem: Vec<Vec<Vec<f64>>>,
}
impl G {
    fn rec(&mut self, a: usize, b: usize, c: usize) -> f64 {
        if a == 100 || b == 100 || c == 100 {
            return 0f64;
        }
        if self.mem[a][b][c] >= 0f64 {
            return self.mem[a][b][c];
        }
        let mut res = 0f64;
        res += (1f64 + self.rec(a + 1, b, c)) * a as f64 / (a + b + c) as f64;
        res += (1f64 + self.rec(a, b + 1, c)) * b as f64 / (a + b + c) as f64;
        res += (1f64 + self.rec(a, b, c + 1)) * c as f64 / (a + b + c) as f64;
        self.mem[a][b][c] = res;
        return self.mem[a][b][c];
    }
}
