#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};
#[fastout]
fn main() {
    input! {
    n:usize,m:usize,
    mut a:[i64;n],
    mut bc:[(usize,i64);m],
    }
    let mut pq = PriorityQueue::new();
    for &x in a.iter() {
        pq.push(x);
    }
    bc.sort_by_key(|&bc| -(bc.1 as i64));
    for &(b, c) in bc.iter() {
        for _ in 0..b {
            if *pq.peek_min().unwrap() >= c {
                break;
            } else {
                pq.pop_min();
                pq.push(c);
            }
        }
    }
    let mut ans = 0;
    while let Some(x) = pq.pop_min() {
        ans += x;
    }
    println!("{}", ans);
}

struct PriorityQueue<T: Ord + Copy> {
    que_max: BinaryHeap<T>,
    que_min: BinaryHeap<Reverse<T>>,
}
impl<T: Ord + Copy> PriorityQueue<T> {
    fn new() -> PriorityQueue<T> {
        PriorityQueue {
            que_max: BinaryHeap::<T>::new(),
            que_min: BinaryHeap::<Reverse<T>>::new(),
        }
    }
    fn is_empty(&mut self) -> bool {
        self.que_max.is_empty()
    }
    fn push(&mut self, x: T) {
        self.que_max.push(x);
        self.que_min.push(Reverse(x));
    }
    fn len(&mut self) -> usize {
        self.que_max.len()
    }
    fn pop_max(&mut self) -> Option<T> {
        self.que_min.pop();
        self.que_max.pop()
    }
    fn pop_min(&mut self) -> Option<T> {
        self.que_max.pop();
        if let Some(x) = self.que_min.pop() {
            let Reverse(y) = x;
            Some(y)
        } else {
            None
        }
    }
    fn peek_max(&mut self) -> Option<&T> {
        self.que_max.peek()
    }
    fn peek_min(&mut self) -> Option<&T> {
        if let Some(x) = self.que_min.peek() {
            let Reverse(y) = x;
            Some(y)
        } else {
            None
        }
    }
}
