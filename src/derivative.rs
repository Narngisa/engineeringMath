#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Dual {
    pub val: f64,
    pub der: f64,
}

impl Dual {
    pub fn new_x(val: f64) -> Self {
        Dual { val, der: 1.0 }
    }

    pub fn new_const(val: f64) -> Self {
        Dual { val, der: 0.0 }
    }

    pub fn powf(self, n: f64) -> Self {
        Dual {
            val: self.val.powf(n),
            der: n * self.val.powf(n - 1.0) * self.der,
        }
    }
}

impl Add for Dual {
    type Output = Dual;
    fn add(self, other: Dual) -> Dual {
        Dual {
            val: self.val + other.val,
            der: self.der + other.der,
        }
    }
}

impl Sub for Dual {
    type Output = Dual;
    fn sub(self, other: Dual) -> Dual {
        Dual {
            val: self.val - other.val,
            der: self.der - other.der,
        }
    }
}

impl Mul for Dual {
    type Output = Dual;
    fn mul(self, other: Dual) -> Dual {
        Dual {
            val: self.val * other.val,
            der: self.val * other.der + other.val * self.der,
        }
    }
}

impl Div for Dual {
    type Output = Dual;
    fn div(self, other: Dual) -> Dual {
        Dual {
            val: self.val / other.val,
            der: (other.val * self.der - self.val * other.der) / (other.val * other.val),
        }
    }
}
