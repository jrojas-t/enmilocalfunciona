// Operaciones Aritmeticas
pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

pub fn resta(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplicacion(a: i32, b: i32) -> i32 {
    a * b
}

pub fn division(a: i32, b: i32) -> i32 {
    a / b
}

pub fn operation(op: &str, num1: i32, num2: i32) -> i32 {
    match op {
        "+" => suma(num1, num2),
        "-" => resta(num1, num2),
        "*" => multiplicacion(num1, num2),
        "/" => division(num1, num2),
        _ => 0,
    }
}
