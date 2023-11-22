
# pre-commit

Es una herramienta que te permite ejecutar las colecciones de postman desde linea de comandos - [más información](https://pre-commit.com/)

**Índice**

- [Pre-requisitos](#Pre-requisitos)
- [Instalación](#Instalación)
- [Configuración](#Configuración)
- [Lenguajes Soportados](#Lenguajes-soportados)
- [Volver a Inicio](/knowmadmood-saco-rust-tokio/README.md)


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

Una vez tengamos instalado pre-commit, abriremos el proyecto en el que lo queramos usar y crearemos el fichero .pre-commit-config.yaml, que contendrá la configuración con la que queremos que la herramienta funcione.

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

Vemos cómo se definen los repositorios de los que se van a tomar los hooks (de momento, solo estamos usando un repositorio) y los respectivos hooks. Se trata de comprobaciones generales, pero podemos ir añadiendo nuevos hooks en función de nuestras preferencias.

También hemos añadido nuevos hooks, tanto del repositorio usado anteriormente como de nuevos repositorios.

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

Una vez que tenemos listo nuestro fichero de configuración, ejecutamos el siguiente comando para configurar los Git Hooks

```bash
pre-commit install
```

Si queremos instalar en otra ciclo que no sea pre-commit ejemplo en pre-push, ejecutar el siguiente comando

```bash
pre-commit install --hook-type pre-commit --hook-type pre-push
```

Con esto, pre-commit estaría listo para mejorar nuestros commits. Sin embargo, si vamos a usarlo en un proyecto ya existente, quizá tenga sentido revisar todo el código actual en lugar de ir revisando los ficheros que van entrando a nuevos commits. Podemos hacer esto con el siguiente comando:

Ejecute con todos los archivos para verificar (Opcional)

```bash
pre-commit run --all-files
```

Tanto ejecutando pre-commit contra todos los ficheros como al hacer un commit, nos mostrará el resultado y las acciones realizadas. En la siguiente captura podemos ver cómo se muestra la fase en la que ha fallado (Fix End of Files) y los ficheros afectados

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
