
# Libreria en Rust

Vamos a crear una libreria que nos ayude hacer operaciones aritmetricas:

- [Volver a Inicio](/knowmadmood-saco-rust-tokio/README.md)

## Instalación ##

La creación de una librería es bastante sencillo, primero que nada vamos a ubicarnos dentro de nuestro proyecto general "kowmadmood-saco-rust-tokio". vamos a ejecutar el siguiente comando:

```bash
cargo new --lib lib-saco-utils
```

Nos genera una estructura de proyecto como esta:

```bash
lib-saco-utils
 /src
     lib.rs
 Cargo.toml
```

Los metodos o operaciones estarán dentro del fichero lib.rs

```rust
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
```


Nuestro fichero Cargo.tml, viene por defecto de la siguiente manera:

```toml
[package]
name = "lib-saco-utils"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

## Referencia de la Libreria al proyecto principal (knowmadmood-saco-rust-tokio) ##

Es muy sencillo hacer referencia, basta con adicionar en el fichero Cargo.toml la referencia de nuestra lib (lib-saco-utils):

```toml
[package]
name = "knowmadmood-saco-rust-tokio"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
...
lib-saco-utils = { version = "*", path = "./lib-saco-utils" }

```
