use trait_enum::trait_enum;

trait Calculator {
    fn call(&self, a: i32, b: i32) -> i32;
}

struct Adder {}
impl Calculator for Adder {
    fn call(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct Subtractor {}
impl Calculator for Subtractor {
    fn call(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

trait_enum! {
    enum Operation: Calculator {
        Adder,
        Subtractor,
    }
}

fn main() {
    let operations = [
        Operation::Adder(Adder {}),
        Operation::Subtractor(Subtractor {}),
    ];

    let a = 10;
    let b = 5;
    for operation in operations {
        let result = operation.call(a, b);
        println!("Result: {}", result);
    }
}
