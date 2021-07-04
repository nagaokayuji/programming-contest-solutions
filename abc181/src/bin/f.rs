#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        mut xy:[(f64,f64);n],
    }

    let isOk = |r: f64| -> bool {
        let mut ind = 0;
        // x: radius
        let mut uf = UnionFind::new(n + 2);
        for (i, &(x, y)) in xy.iter().enumerate() {
            if y as f64 - r * 2. < -100f64 {
                uf.unite(i, n);
            }
            if y as f64 + r * 2. > 100f64 {
                uf.unite(i, n + 1);
            }
        }
        for i in 0..n {
            let (x1, y1) = xy[i];
            for j in i + 1..n {
                let (x2, y2) = xy[j];
                let dif = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
                if ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)) as f64 + eps < r * r * 4f64 {
                    uf.unite(i, j);
                }
            }
        }
        !uf.same(n, n + 1)
    };
    let mut ok = eps;
    let mut ng = 100.1f64;
    while ng - ok > 1e-6 {
        let mut mid = (ng + ok) / 2.;
        if isOk(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
const eps: f64 = 1e-11;
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            par: vec,
            rank: vec![0; n],
            size: vec![1; n],
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    pub fn unite(&mut self, a: usize, b: usize) {
        let apar = self.find(a);
        let bpar = self.find(b);
        if apar == bpar {
            return;
        }
        if self.rank[apar] > self.rank[bpar] {
            self.par[bpar] = apar;
            self.size[apar] += self.size[bpar];
        } else {
            self.par[apar] = bpar;
            self.size[bpar] += self.size[apar];
            if self.rank[apar] == self.rank[bpar] {
                self.rank[bpar] += 1;
            }
        }
    }
    pub fn size(&mut self, a: usize) -> usize {
        let p = self.find(a);
        self.size[p]
    }
}
