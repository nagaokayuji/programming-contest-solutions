#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::{cmp::*, collections::*, mem::*},
};

fn main() {
    input! {q:usize}
    let mut med = 0;
    let mut lower = PriorityQueue::new();
    let mut upper = PriorityQueue::new();
    let mut lower_sum = 0;
    let mut upper_sum = 0;
    let mut bsm = 0i64;
    for _ in 0..q {
        input! {t:usize}
        match t {
            1 => {
                input! {a:i64,b:i64}
                bsm += b;
                if lower.is_empty() {
                    lower.push(a);
                } else {
                    if a < med {
                        lower.push(a);
                        lower_sum += med - a;
                    } else {
                        upper.push(a);
                        upper_sum += a - med;
                    }
                }

                while lower.len() < upper.len() || lower.len() > upper.len() + 1 {
                    if lower.len() < upper.len() {
                        let upper_min = *upper.peek_min().unwrap();
                        let diff = upper_min - med;
                        lower_sum += lower.len() as i64 * diff;
                        upper_sum -= upper.len() as i64 * diff;
                        upper.pop_max();
                        lower.push(upper_min);
                    } else {
                        let lower_max = lower.pop_max().unwrap();
                        let diff = med - lower.peek_max().unwrap();
                        upper.push(lower_max);
                        lower_sum -= lower.len() as i64 * diff;
                        upper_sum += upper.len() as i64 * diff;
                    }
                    med = *lower.peek_max().unwrap();
                }
                med = *lower.peek_max().unwrap();
            }
            _ => {
                println!("{} {}", med, bsm + lower_sum + upper_sum);
            }
        }
    }
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
