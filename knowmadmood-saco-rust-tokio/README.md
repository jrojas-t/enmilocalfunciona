
![Logo](https://upload.wikimedia.org/wikipedia/commons/e/ea/Rust_vector_logo.svg)

[![rust](https://img.shields.io/badge/rust-1.74.0-red)](https://img.shields.io/badge/rust-1.74.0-red)[![postman](https://img.shields.io/badge/postman-latest-orange)](https://img.shields.io/badge/postman-latest-orange)[![docker](https://img.shields.io/badge/docker->4.20-blue)](https://img.shields.io/badge/docker->4.20-blue)[![newman](https://img.shields.io/badge/newman->6.0.0-green)](https://img.shields.io/badge/newman->6.0.0-green)[![git](https://img.shields.io/badge/git-latest-orange)](https://img.shields.io/badge/git-latest-orange)[![crates.io](https://img.shields.io/crates/v/restson.svg)](https://crates.io/crates/restson)[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://raw.githubusercontent.com/spietika/restson-rust/master/LICENSE)[![Swagger: latest](https://img.shields.io/badge/swagger-latest-green.svg)](https://docs.rs/restson/)


**Índice**
- [Introducción](#Introducción)
- [Pre-requisitos](#Pre-requisitos)
- [Montaje de Entorno](#Montaje-de-Entorno)
- [Proceso de Despliegue](#Proceso-de-despliegue)
- [Dockerización](#Dockerización)
- [GitHooks](/knowmadmood-saco-rust-tokio/git/README.md)
- [Newman](/knowmadmood-saco-rust-tokio/newman/collections/README.md)
- [Libreria](/knowmadmood-saco-rust-tokio/lib-saco-utils/README.md)
- [Autor(es)](#Autor)


## Introducción ##

### Proyecto Iniciativa SACO (Squad Aceleradores de COnocimiento) ###

Iniciativa de formación basada en un enfoque agile-squat creada y usada por "knowmad mood" de forma interna y dentro de sus proyectos que proporciona una formación “quirúrgica”: alto rendimiento, necesaria, intensiva y específica.

## Pre-requisitos

 - [Rust v1.74.0](https://www.rust-lang.org/tools/install)
 - [Postman lastet version ](https://www.postman.com/downloads/)
 - [Docker > 4.20](https://www.docker.com/products/docker-desktop/)
 - [Newman v6.0.0](https://www.npmjs.com/package/newman)
 - [Git latest version](https://git-scm.com/downloads)

## Montaje de Entorno ##

Clonar proyecto

```bash
git clone https://github.com/jrojas-t/enmilocalfunciona.git
```

Directorio del proyecto

```bash
cd ./knowmadmood-saco-rust-tokio
```

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

## Autor

- [Jhonatan Rojas Terrones](https://www.linkedin.com/in/jrojast/)
