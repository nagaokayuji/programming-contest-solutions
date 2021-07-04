#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}
fn main() {
    input! {//
        n:usize,
        mut xy:[(i64,i64);n],
    }
    let mut xys = vec![];
    let mut yxs = vec![];
    for (i, &(x, y)) in xy.iter().enumerate() {
        xys.push((x, y, i));
        yxs.push((y, x, i));
    }
    xys.sort();
    yxs.sort();
    let mut edges = vec![];
    for i in 1..n {
        edges.push(Edge {
            from: xys[i - 1].2,
            to: xys[i].2,
            cost: (xys[i - 1].0 - xys[i].0).abs(),
        });
        edges.push(Edge {
            from: yxs[i - 1].2,
            to: yxs[i].2,
            cost: (yxs[i - 1].0 - yxs[i].0).abs(),
        });
    }
    edges.sort_by_key(|x| x.cost);
    let mut ans = 0i64;
    let mut uf = UnionFind::new(n);
    for edge in edges.iter() {
        if !uf.same(edge.from, edge.to) {
            uf.unite(edge.from, edge.to);
            ans += edge.cost;
        }
    }
    println!("{}", ans);
}
pub trait BinarySearch<T> {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize;
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while (ok as i32 - ng as i32).abs() > 1 {
            let mid = (ok + ng) / 2;
            if f(&self[mid as usize]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn lower_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y >= x)
    }
    fn upper_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y > x)
    }
}

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
