// Operaciones Aritmeticas
pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

pub fn division(a: i32, b: i32) -> i32 {
    a / b
}

pub fn operation(op: &str, num1: i32, num2: i32) -> i32 {
    match op {
        "+" => addition(num1, num2),
        "-" => subtraction(num1, num2),
        "*" => multiplication(num1, num2),
        "/" => division(num1, num2),
        _ => 0,
    }
}
