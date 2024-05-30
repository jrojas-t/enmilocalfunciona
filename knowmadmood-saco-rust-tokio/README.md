
![Logo](https://itelligent.es/wp-content/uploads/RUST-lenguaje-programacion.jpeg.webp)

[![rust](https://img.shields.io/badge/rust-1.74.0-red)](https://img.shields.io/badge/rust-1.74.0-red)[![postman](https://img.shields.io/badge/postman-latest-orange)](https://img.shields.io/badge/postman-latest-orange)[![docker](https://img.shields.io/badge/docker->4.20-blue)](https://img.shields.io/badge/docker->4.20-blue)[![newman](https://img.shields.io/badge/newman->6.0.0-green)](https://img.shields.io/badge/newman->6.0.0-green)[![git](https://img.shields.io/badge/git-latest-orange)](https://img.shields.io/badge/git-latest-orange)[![crates.io](https://img.shields.io/crates/v/restson.svg)](https://crates.io/crates/restson)[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://raw.githubusercontent.com/spietika/restson-rust/master/LICENSE)[![Swagger: latest](https://img.shields.io/badge/swagger-latest-green.svg)](https://docs.rs/restson/)

**Índice**
- [Introducción](#Introducción)
- [Pre-requisitos](#Pre-requisitos)
- [Montaje de Entorno](#Montaje-de-Entorno)
- [Proceso de Despliegue](#Proceso-de-despliegue)
- [Dockerización](#Dockerización)
- [Rust desde cero](#Rust-desde-cero)
- [Recomendaciones de Pluggins](#Recomendaciones-de-pluggins)

**Conceptos Adicionales**

- [GitHooks](/knowmadmood-saco-rust-tokio/git/README.md)
- [Newman](/knowmadmood-saco-rust-tokio/newman/collections/README.md)
- [Libreria](/knowmadmood-saco-rust-tokio/lib-saco-utils/README.md)
- [Autor(es)](#Autor)


## Introducción ##

### Proyecto Iniciativa SACO (Squad Aceleradores de COnocimiento) ###

Iniciativa de formación basada en un enfoque agile-squat creada y usada por "knowmad mood" de forma interna y dentro de sus proyectos que proporciona una formación “quirúrgica”: alto rendimiento, necesaria, intensiva y específica.

### Rust ###

Nace como un proyecto personal de uno de los trabajadores de Mozilla (Graydon Hoare) en el año 2006, quien para el año 2010 se apalanca con la fundación (Mozilla) en busca del desarrollo de un lenguaje que facilite la escritura de código con tiempos de ejecución y compilación óptimos, que se encuentre al nivel o por encima de C++, eliminando los inconvenientes con el garbage recollector (recolector de basura) a fin de evitar los problemas con la gestión de memoria.

### Carácteristicas ###

A continuación, te mencionamos algunas de las características de este novedoso lenguaje de programación:

- Ejecución dinámica de seguridad (errores y registros).
- Orientado a Objetos.
- Interfaz simple.
- Gestión automática de guardado.
- Inmutable.
- Compilación nativa y estática.
- Multiplataforma.
- Control de la memoria explícita.
- Permite cadenas UTF8.
- Multiparadigmático.
- Concurrente.

### Ventajas ###

A nivel global, Rust permite desarrollar grandes programas del lado del cliente y del servidor mejorando la calidad del software, sin necesidad de requerir más poder del hardware que lo ejecuta considerando esta como una de las principales ventajas que ofrece. Adicionalmente, gracias al compilador de este, se cumplen convenientemente las garantías de seguridad del resto de las validaciones que conllevan que este lenguaje sea eficiente y seguro.

### Sintaxis de RUST ###

La sintaxis de Rust es muy parecida a la del lenguaje C++, esta cuenta con bloques de código que se encuentran delimitados por llaves, finalizadas las líneas de código por punto y coma (;) y las estructuras de control de flujo cotidianas como lo son: for, if, else, while, do, elseif.

### ¿Que es el Cargo en Rust? ###

Rust viene con una herramienta por defecto para gestionar dependencias, crear proyectos y realizar otras tareas comunes, esta herramienta se llama Cargo.

Esta herramienta ofrece algo similar a lo que sería pip y virtualenv en python, pero además realiza otras tareas como los test y la generación de documentación.

Por lo tanto Cargo sirve para:

- Crear nuevos proyectos Rust a partir de templates (new, init)
- Compilar el proyecto actual (run, build, install)
- Gestionar dependencias del proyecto (search, update)
- Publicar el proyecto en crates.io (publish)
- Generar la documentación del proyecto (doc)
- Ejecutar los tests (test, bench)

## Pre-requisitos
 - [Visual Studio Code - latest version](https://code.visualstudio.com/download)
 - [Rust - v1.74.0](https://www.rust-lang.org/tools/install)
 - [Postman - latest version ](https://www.postman.com/downloads/)
 - [Docker - v4.20](https://www.docker.com/products/docker-desktop/)
 - [Newman v6.0.0](https://www.npmjs.com/package/newman)
 - [Git - latest version](https://git-scm.com/downloads)

## Montaje de Entorno ##

Clonar proyecto

```bash
git clone https://github.com/jrojas-t/enmilocalfunciona.git
```

Directorio del proyecto

```bash
cd ./knowmadmood-saco-rust-tokio
```

Estructura del proyecto

```bash
└── src (Directorio que contiene todas las fuentes de nuestro proyecto)
    └── handler.rs
    └── main.rs
    └── model.rs
    └── response.rs
    └── route.rs
└── tests (Directorio donde contiene los ficheros de tipo test)
    └── operations_test.rs
└── target (Directorio que contiene los ficheros compilados de dependencias, clases, etc)
    └── debug
    └── tmp
├── Cargo.toml (Fichero donde se gestiona las dependencias)
├── Cargo.lock (Fichero que contiene la información exacta sobre las dependencias)
├── README.md (Fichero donde encontramos información del proyecto)
```
Fichero Cargo.toml

```toml
[package]
name = "knowmadmood-saco-rust-tokio"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
axum = { version = "0.6.19" } # Web framework that focuses on ergonomics and modularity.
chrono = { version = "0.4.24", features = ["serde"] } #Library parsing and formatter of the dates
hyper = { version = "0.14.23", features = ["full"] } # A fast and correct HTTP library.
reqwest = { version = "0.11.21", features = ["json"] } #The reqwest crate provides a convenient, higher-level HTTP Client
tokio = { version = "1.22.0", features = ["full"] } # Event-driven, non-blocking I/O platform.
tower = { version =  "0.4.13" } # Modular reusable components for building robust clients and servers.
serde = { version = "1.0.147", features = ["derive"] } # A serialization/deserialization framework.
serde_json = { version = "1.0.89" } # Serde serializion/deserialization of JSON data.
serde_derive = { version = "1.0.188" }
once_cell = { version = "1.16.0" } # Single assignment cells and lazy values.
base64 = { version = "0.21.4" } # Encode and decode base64 as bytes or utf8.
http = { version = "0.2.8" } # Types for HTTP requests and responses.
tower-http = { version = "0.4.0", features = ["cors"] } # Framework for Controle Cors
uuid = { version = "1.3.0", features = ["v4","serde"] } #Generator UUID
lib-saco-utils = { version = "*", path = "./lib-saco-utils" } #External Library Generate for me =)

```

### Tokio para Aplicaciones Asincronar en Rust ###

Tokio es una plataforma de E/S sin bloqueo basada en eventos para escribir aplicaciones asincrónicas con el lenguaje de programación Rust. A alto nivel, proporciona algunos componentes importantes:

Herramientas para trabajar con tareas asincrónicas , incluidas primitivas y canales de sincronización y tiempos de espera, suspensiones e intervalos .
API para realizar E/S asincrónicas , incluidos sockets TCP y UDP , operaciones de sistemas de archivos y gestión de procesos y señales .
Un tiempo de ejecución para ejecutar código asincrónico, que incluye un programador de tareas, un controlador de E/S respaldado por la cola de eventos del sistema operativo (epoll, kqueue, IOCP, etc.) y un temporizador de alto rendimiento.

Tokio consta de una serie de módulos que proporcionan una gama de funcionalidades esenciales para implementar aplicaciones asincrónicas en Rust. En esta sección, haremos un breve recorrido por Tokio, resumiendo las principales API y sus usos.

La forma más sencilla de comenzar es habilitar todas las funciones.

## Proceso de Despliegue ##
>**Importante**
>
>NOTA: Los siguientes comandos se deben ejecutar el directorio raiz del proyecto
>
#### Compilación ####

Para realizar la compilación del proyecto, ejecutamos el siguiente comando:

```bash
cargo build
```

#### Verificación ####

Para realizar la verificación antes de compilar el proyecto, ejecutamos el siguiente comando:

```bash
cargo check
```

#### Despliegue ####

## Proceso de Despliegue ##

Para realizar el desplegar la aplicación en un entorno local, ejecutar el siguiente comando:

>**Importante**
>
>NOTA: Este proceso no se debe realizar en entornos productivos
>

```bash
cargo run
```

## Dockerización ##

Para generar la imagen docker

```bash
docker build -t knowmadmood-saco-rust-tokio . -f .\docker\Dockerfile
```
Para iniciar la aplicación utilizando la imagen generada anteriormente

```bash
docker run -d --name knowmadmood-saco-rust-tokio -p 3000:3000 knowmadmood-saco-rust-tokio
```

Para eliminar la imagen docker

```bash
docker image rm [IMAGE_ID]
```

## Rust desde Cero ##

Esta sección proporciona una idea rápida de la cargo herramienta de línea de comandos. Demostramos su capacidad para generar un nuevo paquete para nosotros, su capacidad para compilar la caja dentro del paquete y su capacidad para ejecutar el programa resultante.

Para crear un nuevo proyecto Rust con Cargo, ejecute el siguiente comando:

```bash
cargo new [PROJECT_NAME]
```

Vemos que cargo nos ha generado la siguiente estructura de proyecto:

```bash
├── Cargo.toml
└── src
    └── main.rs
```

Echamos un vistazo al fichero Cargo.toml

```toml
[package]
name = "[PROJECT_NAME]"
version = "0.1.0"
edition = "2021"

[dependencies]
```
Echamos un vistazo al fichero main.rs

```rust
fn main() {
    println!("Hello, world!");
}
```

## Recomendaciones de Pluggins

- Name: rust-analyzer
Id: rust-lang.rust-analyzer
Description: Rust language support for Visual Studio Code
Version: 0.3.1740
Publisher: The Rust Programming Language
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer

- Name: crates
Id: serayuzgur.crates
Description: Helps Rust developers managing dependencies with Cargo.toml.
Version: 0.6.3
Publisher: Seray Uzgur
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates

- Name: readme
Id: ganlinzhen.readme
Description: readme
Version: 0.0.2
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=ganlinzhen.readme

- Name: GitHooks -- Simple VS Code UI for git hooks.
Id: lakshmikanthayyadevara.githooks
Description: Integrating Git Hooks to vscode UI; Can View, Edit and Run your local Git Hooks which can prevent your git repository from potential issues and enforce code quality.
Version: 1.3.4
Publisher: lakshmikanthayyadevara
VS Marketplace Link: https://marketplace.visualstudio.com/items?itemName=lakshmikanthayyadevara.githooks

## Autor

- [Jhonatan Rojas Terrones](https://www.linkedin.com/in/jrojast/)
