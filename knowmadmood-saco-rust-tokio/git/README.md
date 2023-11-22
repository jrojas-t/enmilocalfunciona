
# pre-commit

Es una herramienta que te permite ejecutar las colecciones de postman desde linea de comandos - [más información](https://pre-commit.com/)

**Índice**

- [Pre-requisitos](#Pre-requisitos)
- [Instalación](#Instalación)
- [Configuración](#Configuración)
- [Lenguajes Soportados](#Lenguajes-soportados)


## Pre-requisitos ##

 - [Python > v3.12.0](https://www.python.org/downloads/)

## Instalación ##

Lo primero que haremos será instalar pre-commit con pip:

```bash
pip install pre-commit
```
Validar la instalación y su versión con:

```bash
pre-commit --version
```

## Configuración ##

- Crear un fichero .pre-commit-config.yaml
- Crear un fichero .pre-commit-hooks.yaml para las revisiones de código personalizadas.
- Puedes generar una configuración muy básica usando pre-commit sample-config
- El conjunto completo de opciones para la configuración se enumeran a continuación.
- Este ejemplo utiliza un formateador para código Rust, sin embargo, pre-commit funciona para cualquier lenguaje de programación.
- Otros [hooks compatibles](https://pre-commit.com/hooks.html).

>**Importante**
>
>Se recomienda crear un directorio especifico para las configuraciones del GitHooks.
> Para este ejemplo se ha creado el directorio (./git/hooks)

Creación del fichero .pre-commit-config.yaml

```yaml
repos:
-   repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v2.3.0
    hooks:
    -   id: check-yaml
    -   id: end-of-file-fixer
    -   id: trailing-whitespace
-   repo: https://github.com/psf/black
    rev: 22.10.0
    hooks:
    -   id: black
```

Creación del fichero .pre-commit-hooks.yaml

```yaml
- id: fmt
  name: fmt
  description: Formatear archivos con cargo fmt.
  entry: cargo fmt
  language: system
  types: [rust]
  args: ["--"]
- id: cargo-check
  name: cargo check
  description: Verifique el paquete en busca de errores.
  entry: cargo check
  language: system
  types: [rust]
  pass_filenames: false

```

Instalación del Hook Script

```bash
pre-commit install
```

Ejecute con todos los archivos para verificar (Opcional)

```bash
pre-commit run --all-files
```

## Lenguajes Soportados ##

pre-commit soporta los siguientes lenguajes de programación:

- [conda](https://pre-commit.com/#conda)
- [coursier](https://pre-commit.com/#coursier)
- [dart](https://pre-commit.com/#dart)
- [docker](https://pre-commit.com/#docker)
- [docker_image](https://pre-commit.com/#docker_image)
- [dotnet](https://pre-commit.com/#dotnet)
- [fail](https://pre-commit.com/#fail)
- [golang](https://pre-commit.com/#golang)
- [haskell](https://pre-commit.com/#haskell)
- [lua](https://pre-commit.com/#lua)
- [node](https://pre-commit.com/#node)
- [perl](https://pre-commit.com/#perl)
- [python](https://pre-commit.com/#python)
- [python_venv](https://pre-commit.com/#python_venv)
- [r](https://pre-commit.com/#r)
- [ruby](https://pre-commit.com/#ruby)
- [rust](https://pre-commit.com/#rust)
- [swift](https://pre-commit.com/#swift)
- [pygrep](https://pre-commit.com/#pygrep)
- [script](https://pre-commit.com/#script)
- [system](https://pre-commit.com/#system)
