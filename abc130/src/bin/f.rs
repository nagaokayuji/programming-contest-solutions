#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
// from:
// https://atcoder.jp/contests/abc130/submissions/5990273

#[fastout]
fn main() {
    input! {
        n:usize,
        xyd:[(f64,f64,char);n]
    }
    let mut xd = D::new();
    let mut yd = D::new();
    for &(x, y, d) in xyd.iter() {
        // x,y どちらも速度を +1 する
        match d {
            'L' => {
                xd.add(x, 0);
                yd.add(y, 1);
            }
            'R' => {
                xd.add(x, 2);
                yd.add(y, 1);
            }
            'U' => {
                xd.add(x, 1);
                yd.add(y, 2);
            }
            _ => {
                xd.add(x, 1);
                yd.add(y, 0);
            }
        }
    }
    // 交差する時間リスト
    let mut ts = vec![0f64];
    for &t in xd.events().iter().chain(yd.events().iter()) {
        ts.push(t);
    }
    let mut ans = 1e18;
    for t in ts {
        let now = xd.get(t) * yd.get(t);
        if ans > now {
            ans = now;
        }
    }
    println!("{}", ans);
}

struct D {
    l: Vec<f64>,
    r: Vec<f64>,
}
impl D {
    fn new() -> D {
        D {
            l: vec![INF; 3], //速度が3通り(0,1,2)
            r: vec![-INF; 3],
        }
    }
    fn add(&mut self, x: f64, v: usize) {
        // 向きごとに最大・最小を格納
        if self.l[v] > x {
            self.l[v] = x;
        }
        if self.r[v] < x {
            self.r[v] = x;
        }
    }
    fn get(&mut self, t: f64) -> f64 {
        // 時刻が与えられたときに幅を出力
        let mut nl = INF;
        let mut nr = -INF;
        for i in 0..3 {
            if nl > self.l[i] + i as f64 * t {
                nl = self.l[i] + i as f64 * t;
            }
            if nr < self.r[i] + i as f64 * t {
                nr = self.r[i] + i as f64 * t;
            }
        }
        nr - nl
    }
    fn events(&mut self) -> Vec<f64> {
        // 交差する時間リストを出力
        let mut ts = Vec::new();
        for i in 0..3 {
            for j in 0..i {
                // 座標の差/相対速度
                let mut t = (self.l[j] - self.l[i]) / (i - j) as f64;
                if t > 0f64 {
                    ts.push(t);
                }
                let mut t = (self.r[j] - self.r[i]) / (i - j) as f64;
                if t > 0f64 {
                    ts.push(t);
                }
            }
        }
        ts
    }
}
const INF: f64 = 1e11;
