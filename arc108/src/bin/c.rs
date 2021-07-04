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
        n:usize,m:usize,
        uvc:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    // let mut gc = vec![vec![];n];
    let mut uf = UnionFind::new(n);
    for &(u, v, c) in uvc.iter() {
        if !uf.same(u, v) {
            g[u].push((v, c));
            g[v].push((u, c));
            uf.unite(u, v);
        }
    }
    let mut q = VecDeque::new();
    let mut ans = vec![0; n];
    q.push_back((0, 0, true));
    while let Some(qq) = q.pop_back() {
        let (now, pr, free) = qq;
        if free {
            ans[now] = if pr == 1 { 2 } else { 1 };
        } else {
            for &nx in g[now].iter() {
                if ans[nx.0] != 0 {
                    ans[now] = nx.1;
                }
            }
        }
        for &nx in g[now].iter() {
            if ans[nx.0] == 0 {
                q.push_back((nx.0, ans[now], nx.1 == ans[now]));
            }
        }
    }

    for &x in ans.iter() {
        println!("{}", x);
    }
}

fn dfs(g: &Vec<Vec<(usize, usize)>>, now: usize, num: usize) {}

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
