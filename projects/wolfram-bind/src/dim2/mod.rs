use std::cmp::{max_by, min_by};
use rand::distributions::Standard;
pub use rand::prelude::*;
pub use crate::utils::norm_minmax;
use itertools::Itertools;

pub struct Points {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
}

impl Iterator for Points {
    type Item = Points;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.next_by(|a, b| (a + b) / 2.0);
        Some(out)
    }
}

impl Points {
    pub fn new(n: usize) -> Self {
        Self::random(n, &mut SmallRng::from_entropy())
    }
    pub fn random(n: usize, rng: &mut SmallRng) -> Self {
        let xs = rng.sample_iter(Standard).take(n).collect();
        let ys = rng.sample_iter(Standard).take(n).collect();
        Self {
            xs,
            ys,
        }
    }
}

impl Points {
    fn minmax_x(&self) -> (f64, f64) {
        unsafe {
            let min = self.xs.iter().cloned().min_by(f64::total_cmp).unwrap_unchecked();
            let max = self.xs.iter().cloned().max_by(f64::total_cmp).unwrap_unchecked();
            (min, max)
        }
    }

    fn minmax_y(&self) -> (f64, f64) {
        unsafe {
            let min = self.ys.iter().cloned().min_by(f64::total_cmp).unwrap_unchecked();
            let max = self.ys.iter().cloned().max_by(f64::total_cmp).unwrap_unchecked();
            (min, max)
        }
    }

    pub fn next_by(&self, f: impl Fn(f64, f64) -> f64) -> Self {
        let mut xs = Vec::with_capacity(self.xs.len());
        let mut ys = Vec::with_capacity(self.ys.len());
        for (a, b) in self.xs.iter().circular_tuple_windows() {
            xs.push(f(*a, *b));
        }
        for (a, b) in self.xs.iter().circular_tuple_windows() {
            ys.push(f(*a, *b));
        }
        Self {
            xs,
            ys,
        }
    }

    pub fn norm(&mut self) {
        let (min_x, max_x) = self.minmax_x();
        let (min_y, max_y) = self.minmax_y();

        let min = min_by(min_x, min_y, f64::total_cmp);
        let max = max_by(max_x, max_y, f64::total_cmp);

        for x in &mut self.xs {
            norm_minmax(x, min, max);
        }
        for y in &mut self.ys {
            norm_minmax(y, min, max);
        }
    }
    pub fn norm_independent(&mut self) {
        let (min_x, max_x) = self.minmax_x();
        let (min_y, max_y) = self.minmax_y();

        for x in &mut self.xs {
            norm_minmax(x, min_x, max_x);
        }

        for y in &mut self.ys {
            norm_minmax(y, min_y, max_y);
        }
    }
}
