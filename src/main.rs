mod derivative;
mod integrate;

use derivative::Dual;

fn main() {
    let x = Dual::new_x(2.0); // x = 2.0, x' = 1.0
    let c3 = Dual::new_const(3.0);
    let c1 = Dual::new_const(1.0);

    // f(x) = (x^2 + 3x) / (x + 1)
    let fx = (x.powf(2.0) + (c3 * x)) / (x + c1);

    println!("Value f(2): {}", fx.val); // 10.0 / 3.0 ≈ 3.3333
    println!("Derivative f'(2): {}", fx.der); // (7*3 - 10*1) / 9 = 11/9 ≈ 1.2222
}
