
# Módulo Base (src)

En este módulo base encontraremos los ficheros rust (*.rs) que será utilizado en tiempo de ejecución del proyecto.

**Índice**
- [Creación del proyecto](#Creacion-del-proyecto)
- [Ficheros](#Ficheros)

>**Importante**
>
>Este comando se debe ejecutar en un directorio vacio
>
>NOTA: Solo se debe utilizar cuando se requiere crear el proyecto (solo una única vez).

## Creación del Proyecto ##

Para la creación de un nuevo proyecto en rust utilizaremos Cargo (herramienta que se utiliza para la gestión de dependencia, creación de proyectos y realizar tareas comunes, esta herramienta seria el equivalente el "Pip" en Python).

```bash
cargo new [PROJECT_NAME]
cd ./[PROJECT_NAME]
```

## Ficheros
Al ejecutar el comando anterior se creará los siguientes módulos

### ./src ###
- handler.rs: Es la clase donde se encontrará la logica del negocio, validaciones de sintaxis, gestión de errores, etc.

- main.rs: Es la clase donde se encuentra el metodo principal de la aplicación a ejecutar main().

- model.rs: Es la clase modelo donde iremos a crear las entidades o modelos utilizados según el caso de uso.

- response.rs: Es la clase donde crearemos las entidades denomidos payloads de tipo response, para el retorno de información al realizar una petición.

- route.rs: Es la clase donde se encontrará el mapeo de endpoints a ser publicados.
