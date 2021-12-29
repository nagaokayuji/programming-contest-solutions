#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());
    let n = sc.usize();
    let q = sc.usize();
    let a: Vec<i64> = sc.vec(n);

    let lrv = (0..q).map(|_| sc.vec::<i64>(3)).collect::<Vec<_>>();
    // println!("{:?}", lrv);
    let mut ret = 0i64;
    let mut ads = vec![0i64; n + 1];
    let mut sa = vec![];
    for i in (0..n - 1) {
        ret += (a[i] - a[i + 1]).abs();
        sa.push(a[i + 1] - a[i]);
    }
    // println!("{:?}", sa);
    for q in &lrv {
        let (l, r, v) = (q[0], q[1], q[2]);
        let l = (l - 1) as usize;
        let r = (r - 1) as usize;
        let prev =
            if l > 0 { sa[l - 1].abs() } else { 0 } + if r < n - 1 { sa[r].abs() } else { 0 };
        if l > 0 {
            sa[l - 1] += v;
        }
        if r < n - 1 {
            sa[r] -= v
        }
        let now = if l > 0 { sa[l - 1].abs() } else { 0 } + if r < n - 1 { sa[r].abs() } else { 0 };
        ret += now - prev;

        println!("{}", ret);
    }
}
static MX: usize = 1010101;
static MOD: i64 = 1000000007;
static INF: i64 = std::i64::MAX >> 1;

struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);
impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    fn writeln<S: ToString>(&mut self, s: S) {
        self.write(format!("{}\n", s.to_string()));
    }
    fn writevec<T: ToString>(&mut self, v: &[T]) {
        let s = v
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        self.writeln(format!("{} ", &s));
    }
    fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    fn usize(&mut self) -> usize {
        self.read::<usize>()
    }
    fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    fn i32(&mut self) -> i32 {
        self.read::<i32>()
    }
    fn i64(&mut self) -> i64 {
        self.read::<i64>()
    }
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    fn vecn<T: std::str::FromStr>(&mut self) -> Vec<T> {
        let n: usize = self.read();
        self.vec(n)
    }
    fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
