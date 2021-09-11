#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

#[fastout]
fn main() {
    input! {//
        n:usize,k:usize,
        v:[i64;n],
    }
    let mut q = MinQueue::new();

    let mut ans = 0;
    for leftp in 0..=min(n, k) {
        for rightp in 0..=min(n, k) - leftp {
            let pushc = k - leftp - rightp; //捨てられる数
            for l in 0..leftp {
                q.push(v[l]);
            }
            for r in 0..rightp {
                q.push(v[n - r - 1]);
            }
            for _ in 0..pushc {
                if let Some(x) = q.peek() {
                    if x < &0 {
                        q.pop();
                    }
                }
            }
            let mut ret = 0;
            while let Some(x) = q.pop() {
                ret += x;
            }
            ans = max(ret, ans);
        }
    }
    println!("{}", ans);
}

use std::{cmp::*, collections::*};
struct MinQueue<T: Ord + Copy> {
    que: BinaryHeap<Reverse<T>>,
}
impl<T: Ord + Copy> MinQueue<T> {
    fn new() -> MinQueue<T> {
        MinQueue {
            que: BinaryHeap::<Reverse<T>>::new(),
        }
    }
    fn is_empty(&mut self) -> bool {
        self.que.is_empty()
    }
    fn push(&mut self, x: T) {
        self.que.push(Reverse(x));
    }
    fn len(&mut self) -> usize {
        self.que.len()
    }
    fn pop(&mut self) -> Option<T> {
        if let Some(x) = self.que.pop() {
            let Reverse(y) = x;
            Some(y)
        } else {
            None
        }
    }
    fn peek(&mut self) -> Option<&T> {
        if let Some(x) = self.que.peek() {
            let Reverse(y) = x;
            Some(y)
        } else {
            None
        }
    }
}
