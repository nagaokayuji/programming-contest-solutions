#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
/*
    ■
*/
const eps: f64 = 1e-1;
const pi: f64 = 3.14159265358979323846264338327950288419716939937510582097494459230781640628;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D(f64, f64);
impl Vector2D {
    pub fn add(a: f64, b: f64) -> f64 {
        let c = a + b;
        if c.abs() < eps {
            0.0
        } else {
            c
        }
    }
    fn degree(self) -> f64 {
        let mut ret = self.1.atan2(self.0);
        if ret + eps < 0. {
            ret += 2. * pi;
        }
        ret
    }
    pub fn dot(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.0, self.1 * other.1)
    }
    pub fn det(self, other: Vector2D) -> f64 {
        Self::add(self.0 * other.1, -self.1 * other.0)
    }
    pub fn length(&self) -> f64 {
        f64::sqrt((self.0).powi(2) + (self.1).powi(2))
    }
    pub fn unit(self) -> Vector2D {
        let l = self.length();
        Vector2D(self.0 / l, self.1 / l)
    }
    pub fn normal(self) -> Vector2D {
        Vector2D(self.1, -self.0)
    }
}
impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, rhs.0), Vector2D::add(self.1, rhs.1))
    }
}
impl std::ops::Sub for Vector2D {
    type Output = Vector2D;
    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D(Vector2D::add(self.0, -rhs.0), Vector2D::add(self.1, -rhs.1))
    }
}
impl std::ops::Mul<f64> for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2D(rhs * self.0, rhs * self.1)
    }
}
impl std::ops::Div<f64> for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: f64) -> Self::Output {
        Vector2D(self.0 / rhs, self.1 / rhs)
    }
}
fn main() {
    input! {//
        n:usize,
        mut xy:[(f64,f64);n],
    }
    let mut xy = xy.iter().map(|&(x, y)| Vector2D(x, y)).collect::<Vec<_>>();
    let mut ans = 0.;
    for &base in xy.iter() {
        // dbg!(&base.degree());
        let mut now = Vector2D(0., 0.);
        for i in 0..n {
            if (base.degree() - xy[i].degree()).abs() * 2. < pi + eps + eps {
                now = now + xy[i];
            }
            if ans < now.length() {
                ans = now.length();
            }
        }
    }

    println!("{:.12}", ans);
}
