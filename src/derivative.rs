#![allow(dead_code)]

pub fn diff_1(_c: f64) -> f64 {
    0.0
}

pub fn diff_2(a: f64, _x: &str) -> f64 {
    a
}

pub fn diff_3(c: f64, u: &str) -> f64 {
    c * diff_2(1.0, u)
}

pub fn diff_4_add(du: f64, dv: f64) -> f64 {
    du + dv
}

pub fn diff_4_sub(du: f64, dv: f64) -> f64 {
    du - dv
}

pub fn diff_5(u: f64, du: f64, v: f64, dv: f64) -> f64 {
    u * dv + v * du
}

pub fn diff_6(u: f64, du: f64, v: f64, dv: f64) -> f64 {
    (v * du - u * dv) / (v * v)
}