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
        mut ab:[(Usize1,Usize1);n-1],
        q:usize,
        mut tex:[(usize,Usize1,i64);q],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        g[a].push(b);
        g[b].push(a);
    }
    let mut lca = LCA::new(&g);
    let mut dp = vec![0i64; n];
    for &(t, e, x) in tex.iter() {
        let (a, b) = if t == 1 { ab[e] } else { (ab[e].1, ab[e].0) };
        if lca.depth[a] > lca.depth[b] {
            dp[a] += x;
        } else {
            dp[0] += x;
            dp[b] -= x;
        }
    }
    let mut ans = vec![0i64; n];
    ans[0] = dp[0];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        for &u in g[v].iter() {
            if lca.depth[u] < lca.depth[v] {
                continue;
            }
            ans[u] = ans[v] + dp[u];
            q.push_back(u);
        }
    }
    for &x in ans.iter() {
        println!("{}", x);
    }
}
struct G {
    g: Vec<Vec<usize>>,
    ls: Vec<usize>,
    rs: Vec<usize>,
    ind: usize,
}
impl G {
    fn new(g: &Vec<Vec<usize>>) -> G {
        let n = g.len();
        return G {
            g: g.clone(),
            ls: vec![0; n],
            rs: vec![0; n],
            ind: 0,
        };
    }
    fn dfs(&mut self, v: usize, p: usize) {
        self.ls[v] = self.ind;
        self.ind += 1;
        for &nx in &self.g[v].clone() {
            if p != nx {
                self.dfs(nx, v);
            }
        }
        self.rs[v] = self.ind;
    }
}
pub struct LCA {
    pub depth: Vec<usize>,
    pub parent: Vec<Vec<Option<usize>>>,
}
impl LCA {
    pub fn new(g: &[Vec<usize>]) -> LCA {
        LCA::with_root(0, g)
    }
    pub fn with_root(root: usize, g: &[Vec<usize>]) -> LCA {
        fn dfs(
            i: usize,
            p: Option<usize>,
            d: usize,
            g: &[Vec<usize>],
            depth: &mut [usize],
            parent: &mut [Vec<Option<usize>>],
        ) {
            parent[i][0] = p;
            depth[i] = d;
            for &t in &g[i] {
                if Some(t) != p {
                    dfs(t, Some(i), d + 1, g, depth, parent);
                }
            }
        }
        let n = g.len();
        let l2 = (1..).find(|i| 1usize << i > n).unwrap();
        let mut depth = vec![0; n];
        let mut parent = vec![vec![None; l2 + 1]; n];
        dfs(root, None, 0, &g, &mut depth, &mut parent);
        for i in 1..l2 + 1 {
            for j in 0..n {
                if let Some(p) = parent[j][i - 1] {
                    parent[j][i] = parent[p][i - 1];
                }
            }
        }
        LCA { depth, parent }
    }
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        use std::mem::swap;
        if self.depth[b] < self.depth[a] {
            swap(&mut a, &mut b);
        }
        while self.depth[a] != self.depth[b] {
            b = self.parent[b][(self.depth[b] - self.depth[a]).trailing_zeros() as usize].unwrap();
        }
        if a == b {
            return a;
        }
        for i in (0..self.parent[0].len()).rev() {
            if self.parent[a][i] != self.parent[b][i] {
                a = self.parent[a][i].unwrap();
                b = self.parent[b][i].unwrap();
            }
        }
        self.parent[a][0].unwrap()
    }
}
