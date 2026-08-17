pub fn derivative<F>(f: F, x: f64 ,h: f64) -> f64 
where F: Fn(f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}