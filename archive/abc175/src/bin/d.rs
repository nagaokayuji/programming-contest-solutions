#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:usize,
        p:[Usize1;n],
        c:[i64;n],
    }
    let mut uf = UnionFind::new(n + 1);
    for i in 0..n {
        uf.unite(i, p[i]);
    }

    let mut used = vec![false; n];
    const INF: i64 = 1 << 60;
    let mut ans = -INF;

    for i in 0..n {
        let mut ret = -INF;
        let pa = uf.find(i);
        if used[pa] {
            continue;
        }
        used[pa] = true;
        let mut v = vec![];
        let mut now = pa;
        v.push(pa);
        while p[now] != pa {
            v.push(p[now]);
            now = p[now];
        }
        let len = v.len();
        let mut lns = vec![0i64; len * 2 + 1];
        lns[0] = 0;
        for i in 1..len * 2 + 1 {
            lns[i] = lns[i - 1] + c[p[v[(i - 1) % len]]];
        }
        for st in 1..=len {
            let mut k = k;
            let r = lns[st + len] - lns[st];
            let mut q = (k / len) as i64;
            if r <= 0 {
                let mut mxp = -INF;
                for pl in 0..min(len, k) {
                    mxp = max(mxp, lns[st + pl] - lns[st - 1]);
                }
                ret = max(ret, mxp);
            } else {
                if q == 0 {
                    let mut mxp = 0;
                    for pl in 0..min(len, k) {
                        mxp = max(mxp, lns[st + pl] - lns[st - 1]);
                    }
                    ret = max(ret, mxp);
                } else {
                    let mut tmp = 0;
                    tmp += (q - 1) * r;
                    let mut mxp = 0i64;
                    for pl in 0..k % len {
                        mxp = max(mxp, lns[st + pl] - lns[st - 1]);
                    }
                    ret = max(ret, tmp + r + mxp);
                    mxp = -INF;
                    for pl in 0..len {
                        mxp = max(mxp, lns[st + pl] - lns[st - 1]);
                    }
                    ret = max(ret, tmp + mxp);
                }
            }
        }
        ans = max(ans, ret);
    }
    println!("{}", ans);
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
