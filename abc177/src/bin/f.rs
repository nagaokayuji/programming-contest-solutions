#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        h:usize,w:usize,
        ab:[(Usize1,Usize1);h],
    }
    let mut dp = vec![0; w];
    let mut seg =
        RangeAddSegmentTree::new(w + 5, INF as usize, |a, b| if a > b { b } else { a }, 0);
    let mut ans = vec![0; h];
    for i in 0..w {
        seg.update(i, 0);
    }
    let mut left = 0;
    let mut right = w - 1;

    for k in 0..h {
        let mut l = ab[k].0;
        let mut r = ab[k].1;
        if (l..=r).contains(&left) {
            left = r + 1;
        }
        seg.add(l..r + 1, 1);
        if seg.get(left..w) > 0 {
            ans[k] = k + 1;
            continue;
        }
    }
    dbg!(&ans);
    for i in 0..h {
        if ans[i] >= INF as usize {
            println!("{}", -1);
        } else {
            println!("{}", ans[i]);
        }
        // dbg!(seg.get(i..i + 1));
    }
}
const INF: i64 = 1 << 60;
type Range = std::ops::Range<usize>;
pub struct RangeAddSegmentTree<T, F> {
    data: Vec<T>,
    lazy: Vec<T>,
    size: usize,
    f: F,
    init: T,
}
impl<T, F> RangeAddSegmentTree<T, F>
where
    T: PartialOrd + ::std::ops::Add<Output = T> + ::std::ops::Sub<Output = T> + Copy,
    F: Fn(T, T) -> T + Copy,
{
    pub fn new(n: usize, init: T, f: F, zero: T) -> Self {
        let size = n.next_power_of_two();
        RangeAddSegmentTree {
            data: vec![init; size * 2],
            lazy: vec![zero; size * 2],
            size,
            init,
            f,
        }
    }
    pub fn add(&mut self, range: Range, value: T) {
        self.add_to_range(range, value, 0, 0..self.size);
    }
    fn add_to_range(&mut self, add_range: Range, value: T, mut k: usize, seg_range: Range) {
        if add_range.end <= seg_range.start || seg_range.end <= add_range.start {
            return;
        }
        if add_range.start <= seg_range.start && seg_range.end <= add_range.end {
            self.lazy[k] = self.lazy[k] + value;
            while k > 0 {
                k = (k - 1) / 2;
                self.data[k] = (self.f)(
                    self.data[k * 2 + 1] + self.lazy[k * 2 + 1],
                    self.data[k * 2 + 2] + self.lazy[k * 2 + 2],
                );
            }
        } else {
            let mid = (seg_range.start + seg_range.end) / 2;
            self.add_to_range(add_range.clone(), value, k * 2 + 1, seg_range.start..mid);
            self.add_to_range(add_range, value, k * 2 + 2, mid..seg_range.end);
        }
    }
    pub fn update(&mut self, pos: usize, value: T) {
        let cur = self.get(pos..(pos + 1));
        let mut k = pos + self.size - 1;
        let raw = self.data[k];
        self.data[k] = raw + value - cur;
        while k > 0 {
            k = (k - 1) / 2;
            self.data[k] = (self.f)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
        }
    }
    pub fn get(&self, range: Range) -> T {
        self.get_from_range(range, 0, 0..self.size)
    }
    fn get_from_range(&self, get_range: Range, k: usize, seg_range: Range) -> T {
        if get_range.end <= seg_range.start || seg_range.end <= get_range.start {
            self.init
        } else if get_range.start <= seg_range.start && seg_range.end <= get_range.end {
            self.data[k] + self.lazy[k]
        } else {
            let mid = (seg_range.start + seg_range.end) / 2;
            let x = self.get_from_range(get_range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.get_from_range(get_range, k * 2 + 2, mid..seg_range.end);
            (self.f)(x, y) + self.lazy[k]
        }
    }
}
