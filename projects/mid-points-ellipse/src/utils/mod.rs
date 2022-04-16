/// \frac{2t-(\max+\min)} / {\max-\min}
pub fn norm_minmax(t: &mut f64, min: f64, max: f64) {
    let a = max + min;
    let b = max - min;
    *t = (2.0 * *t - a) / b
}

