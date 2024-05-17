use std::{rc::Rc, sync::Arc};

use auto_impl::auto_impl;

#[auto_impl(&, Box, Rc, Arc)]
trait Calculator {
    fn call(&self, a: i32, b: i32) -> i32;
}

#[derive(Clone, Copy)]
struct Adder {}
impl Calculator for Adder {
    fn call(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct CalculatorHolder<T: Calculator> {
    calculator: T,
}

fn main() {
    let adder = Adder {};
    let holder1 = CalculatorHolder { calculator: adder };
    let holder2 = CalculatorHolder { calculator: &adder };
    let holder3 = CalculatorHolder {
        calculator: Box::new(adder),
    };
    let holder4 = CalculatorHolder {
        calculator: Rc::new(adder),
    };
    let holder5 = CalculatorHolder {
        calculator: Arc::new(adder),
    };
    let a = 10;
    let b = 5;

    let result = holder1.calculator.call(a, b);
    println!("Result: {}", result);

    let result = holder2.calculator.call(a, b);
    println!("Result: {}", result);

    let result = holder3.calculator.call(a, b);
    println!("Result: {}", result);

    let result = holder4.calculator.call(a, b);
    println!("Result: {}", result);

    let result = holder5.calculator.call(a, b);
    println!("Result: {}", result);
}
