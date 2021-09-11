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
        n:usize,
        mut xy:[(i64,i64);n],
        m:usize,
    }

    //
    // lef -> (x,y) -> (y,-x)
    // rig -> (x,y) -> (-y,x)
    // x=p -> (x,y) -> (2*p-x,y)
    // y=p -> (x,y) -> (x, 2*p-y)

    let mut e = (false, 1i64, 1i64, 0i64, 0i64);

    let mut cnv = vec![];
    cnv.push(e.clone());
    let mut prv = e;
    let mut iv = false;
    for _ in 0..m {
        input! {o:usize}
        match o {
            1 => {
                iv = !iv;
                cnv.push((iv, prv.2, -1 * prv.1, prv.3, prv.4));
            }
            2 => {
                iv = !iv;
                cnv.push((iv, -prv.2, prv.1, prv.3, prv.4));
            }
            3 => {
                input! {p:i64};
                cnv.push((
                    iv,
                    prv.1 + 2 * p,
                    prv.2,
                    prv.3 - if !iv { 1 } else { 0 },
                    prv.4 - if !iv { 0 } else { 1 },
                ));
            }
            4 => {
                input! {p:i64};
                cnv.push((
                    iv,
                    prv.1,
                    prv.2 + 2 * p,
                    prv.3 - if !iv { 0 } else { 1 },
                    prv.4 - if !iv { 1 } else { 0 },
                ));
            }
            _ => {}
        };
        prv = cnv.last().unwrap().clone();
    }
    input! {q:usize,
    ab:[(usize,Usize1);q]}

    for &(a, b) in ab.iter() {
        let query = cnv[a];

        let (x, y) = if iv { (xy[b].1, xy[b].0) } else { xy[b] };
        println!(
            "{} {}",
            xy[b].0 + query.1 + 2 * x * query.3 - 1,
            xy[b].1 + query.2 + 2 * y * query.4 - 1
        );
    }
    dbg!(&cnv);
}
#[derive(Clone)]
struct Matrix {
    m: Vec<Vec<i64>>,
}
impl Matrix {
    fn new(v: Vec<Vec<i64>>) -> Matrix {
        Matrix { m: v }
    }
    fn set(&mut self, v: Vec<Vec<i64>>) {
        self.m = v.clone();
    }
    fn dot(&self, m: &Matrix, md: i64) -> Vec<Vec<i64>> {
        let a = self.m.len();
        let b = self.m[0].len();
        let c = m.m[0].len();
        let mut ret = vec![vec![0; c]; a];
        for i in 0..a {
            for j in 0..c {
                for k in 0..b {
                    ret[i][j] += (self.m[i][k] % md) * (m.m[k][j] % md) % md;
                }
            }
        }
        ret
    }
    fn pow(&mut self, k: usize, md: i64) -> Vec<Vec<i64>> {
        let n = self.m.len();
        let mut y = vec![vec![0; n]; n];
        for i in 0..n {
            y[i][i] = 1;
        }
        let mut z = self.m.clone();
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                y = Matrix::new(y).dot(&Matrix::new(z.clone()), md);
            }
            z = Matrix::new(z.clone()).dot(&Matrix::new(z.clone()), md);
            k >>= 1;
        }
        y
    }
}
