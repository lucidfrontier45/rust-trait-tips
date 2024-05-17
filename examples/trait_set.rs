use trait_set::trait_set;

trait_set! {
    trait ThreadSafeCalculationFn = Fn(i32, i32) -> i32 + Sync + Send;
}

fn execute_calculation_fn(calculation_fn: &impl ThreadSafeCalculationFn, a: i32, b: i32) -> i32 {
    calculation_fn(a, b)
}

fn main() {
    let adder = |a, b| a + b;
    let subtractor = |a, b| a - b;

    let a = 10;
    let b = 5;

    let result = execute_calculation_fn(&adder, a, b);
    println!("Result: {}", result);

    let result = execute_calculation_fn(&subtractor, a, b);
    println!("Result: {}", result);
}
