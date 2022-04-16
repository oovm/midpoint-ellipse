pub use rand::prelude::*;
pub use crate::utils::norm_minmax;

pub struct Points {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
}

impl Points {
    pub fn norm(&mut self) {}
    pub fn norm_independent(&mut self) {
        let max_x = self.xs.max();
        let min_x = self.xs.min();

        let min_y = self.ys.min();
        let max_y = self.ys.max();

        for mut x in &mut self.xs {
            norm_minmax(x, &min_x, &max_x);
        }
        for mut y in &mut self.ys {
            *y = (*y - min_y) / y_range;
        }
    }
}
