#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {// //
           n:usize,q:usize,
        abcd:[(Usize1,Usize1,Usize1,usize);n-1],
        xyuv:[(Usize1,usize,Usize1,Usize1);q],
    }

    /*
        lca(u,v) = a とすると
        dist(u,v) = dist[u]+dist[v] - 2dist[a]

        色 x_i のすべての辺の長さが y_i に変更されたときの dist[u_i]
        , dist[v_i]
        , dist[a_i] を求める。
    */
    let mut g = vec![vec![]; n];
    let mut e = vec![vec![]; n];
    let mut G = vec![vec![]; n];
    for (i, &(a, b, c, d)) in abcd.iter().enumerate() {
        g[a].push(b);
        g[b].push(a);
        e[a].push((b, d, c));
        e[b].push((a, d, c));
        G[a].push((b, i));
        G[b].push((a, i));
    }
    let mut lca = LCA::new(&g);
    // lca.lca(u,v)
    let mut dist = vec![!0; n];
    let mut que = Vec::new();
    que.push((0, 0));
    while let Some(x) = que.pop() {
        let (now, dis) = x;
        dist[now] = dis;
        for &(nb, nd, nc) in e[now].iter() {
            if dist[nb] == !0 {
                que.push((nb, dis + nd));
            }
        }
    }
    dbg!(&dist);
    // dbg!(&lca.lca(3, 4));
    // dbg!(&lca.lca(2, 4));
    // dbg!(&lca.lca(1, 4));
    let mut W = vec![vec![]; n];
    for (i, &(x, y, u, v)) in xyuv.iter().enumerate() {
        W[u].push((i, x, y, 1));
        W[v].push((i, x, y, 1));
        W[lca.lca(u, v)].push((i, x, y, -2));
    }
    let mut d = D {
        W: W,
        ans: vec![0; q],
        sum: vec![0; n],
        cnt: vec![0; n],
        e: e,
    };
    d.dfs(0, !0, 0);
    // d.dfs(0, !0);
    for &a in d.ans.iter() {
        println!("{}", a);
    }

    // let mut ans = vec![0; q];
}
struct D {
    // u: usize,
    // p: usize,
    W: Vec<Vec<(usize, usize, usize, i64)>>,
    ans: Vec<i64>,
    sum: Vec<i64>,
    cnt: Vec<i64>,
    e: Vec<Vec<(usize, usize, usize)>>,
}
impl D {
    fn dfs(&mut self, v: usize, pre: usize, dep: usize) {
        for &(nx, dis, col) in self.e[v].clone().iter() {
            if nx == pre {
                continue;
            }
            self.cnt[col] += 1;
            self.sum[col] += dis as i64;
            self.dfs(nx, v, dep + dis);
            self.cnt[col] -= 1;
            self.sum[col] -= dis as i64;
        }
        for &(idx, col, dist, cf) in self.W[v].iter() {
            self.ans[idx] += cf * (dep as i64 + -self.sum[col] + self.cnt[col] * dist as i64);
        }
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
