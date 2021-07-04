#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        n:usize,
        m:usize,
        mut a:[i64;n],
        mut b:[i64;n],
        cd:[(Usize1,Usize1);m],
    }

    // let asum = a.iter().fold(0, |sum, x| sum + x);
    // let bsum = b.iter().fold(0, |sum, x| sum + x);
    // if asum != bsum {
    //     println!("{}", "No");
    //     return;
    // }

    // let mut g = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for &(c, d) in cd.iter() {
        // g[c].push(d);
        // g[d].push(c);
        uf.unite(c, d);
    }
    // let mut vs = vec![vec![]; n];
    // let mut renketsu = v
    let mut ass = vec![0; n];
    let mut bss = vec![0; n];
    for i in 0..n {
        let root = uf.find(i);
        // vs[root].push(i);
        ass[root] += a[i];
        bss[root] += b[i];
    }
    // for i in 0..n {
    //     if vs[i].len() > 0 {
    //         // i = root
    //         let mut asum = 0i64;
    //         let mut bsum = 0i64;
    //         dbg!(&vs[i]);
    //         for &x in vs[i].iter() {
    //             asum += a[x];
    //             bsum += b[x];
    //         }
    //         if asum != bsum {
    //             println!("{}", "No");
    //             return;
    //         }
    //     }
    // }
    for i in 0..n {
        if ass[i] != bss[i] {
            println!("No");
            return;
        }
    }
    println!("{}", "Yes");
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
