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
        mut xy:[(usize,usize);n],
    }
    xy.sort();
    let mut uf = UnionFind::new(MX * 2);
    for &(x, y) in xy.iter() {
        uf.unite(x, y + MX);
    }
    let mut cx = vec![0; MX * 2 + 1];
    let mut cy = vec![0; MX * 2 + 1];

    for i in 0..MX {
        cx[uf.find(i)] += 1;
    }
    for i in MX..MX * 2 {
        cy[uf.find(i)] += 1;
    }
    let mut c = (0..MX * 2).fold(0, |sum, x| sum + cx[x] * cy[x]);
    println!("{}", c - n);
}
const MX: usize = 110101;

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
    pub fn groups_map(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut ret = BTreeMap::new();
        for i in 0..self.par.len() {
            let parent = self.find(i);
            ret.entry(parent).or_insert(vec![]).push(i);
        }
        ret
    }
}
