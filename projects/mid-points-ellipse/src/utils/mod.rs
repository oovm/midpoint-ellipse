pub fn norm_minmax(x: &mut f64, min: f64, max: f64) {
    let a = max + min;
    let b = max - min;
    *x = (2 * x - a) / b
}