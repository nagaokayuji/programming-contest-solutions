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
        n:usize,q:usize,
        mut c:[Usize1;n],
        q:[(usize,Usize1,Usize1);q],
    }
    let mut mp = vec![BTreeMap::new(); n];
    for i in 0..n {
        *mp[i].entry(c[i]).or_insert(0) += 1;
    }
    let mut uf = UnionFind::new(n, mp);
    for &(q, a, b) in q.iter() {
        if q == 1 {
            uf.unite(a, b);
        } else {
            let root = uf.find(a);
            println!("{}", uf.mp[root].get(&b).unwrap_or(&0));
        }
    }
}
pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
    mp: Vec<BTreeMap<usize, i64>>,
}
impl UnionFind {
    pub fn new(n: usize, mp: Vec<BTreeMap<usize, i64>>) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            par: vec,
            rank: vec![0; n],
            size: vec![1; n],
            mp: mp,
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
            // aparにつける
            let bmp = self.mp[bpar].clone();
            for (k, v) in bmp {
                *self.mp[apar].entry(k).or_insert(0) += v;
            }
        } else {
            self.par[apar] = bpar;
            self.size[bpar] += self.size[apar];
            if self.rank[apar] == self.rank[bpar] {
                self.rank[bpar] += 1;
            }
            let amp = self.mp[apar].clone();
            for (k, v) in amp {
                *self.mp[bpar].entry(k).or_insert(0) += v;
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
