mod diff;

fn main() {

    let my_func = |x: f64| (x * x) + (3.0 * x);
    let result = diff::derivative(my_func, 2.0, 1e-6);
    println!("Derivative result: {result}");
}
