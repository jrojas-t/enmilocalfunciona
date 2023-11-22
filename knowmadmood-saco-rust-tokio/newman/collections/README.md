
# Newman

Es una herramienta que te permite ejecutar las colecciones de postman desde linea de comandos.

**Índice**
- [Pre-requisitos](#Pre-requisitos)
- [Instalación](#Instalacion)
- [Ejecutar](#Ejecutar)
- [Opciones de Reporte](#Opciones-de-reporte)

## Pre-requisitos ##

 - [NodeJS > v20.2.0](https://nodejs.org/en/download)
 - [Npm > v9.7.0](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

## Instalación ##

Newman es un paquete de Node.js y, por lo tanto, lo podemos localizar en su gestor de paquetes npm (Node Package Manager).

Para poder disfrutar de él debemos instalarlo como cualquier otro paquete de node.

```bash
npm install -g newman
```
Instalación de la extensión para la generación de Reportes en HTML

```bash
npm install -g newman-reporter-htmlextra
```

## Ejecutar ##

Para ejecutar enviale environment por entorno

```bash
newman run ./newman/collections/API_Rust.postman_collection.json -e ./newman/collections/environment/Local-API_Rust_Environment.postman_environment.json
```

Generando reporte en HTML

```bash
newman run ./newman/collections/API_Rust.postman_collection.json -e ./newman/collections/environment/Local-API_Rust_Environment.postman_environment.json -r htmlextra
```


## Opciones de Reporte ##

Si necesitamos tener un informe que muestre de forma específica y clara tanto la ejecución de los tests, como los resultados de los mismos.

Con la opción “-r” o “--reporters” indicamos que tipo de reporte queremos: cli, json, junit, progress o emojitrain.

Cli es la opción por defecto y, a su vez, consta de una gran variedad de opciones que podemos ir seleccionando según nuestras necesidades.

- reporter-cli-silent: el reporte es deshabilitado internamente y no se muestra salida en el terminal.
- reporter-cli-no-summary: la tabla de resumen estadístico no se muestra.
- reporter-cli-no-failures: evita que los fallos de ejecución se impriman por separado.
- reporter-cli-no-assertions: desactiva la salida de las aserciones a medida que ocurren.
- reporter-cli-no-success-assertions: desactiva la salida para aserciones exitosas a medida que ocurren.
- reporter-cli-no-console: desactiva la salida de console.log (y otras llamadas a la consola) de los scripts de la colección.
- reporter-cli-no-banner: desactiva el banner de Newman que se muestra al comienzo de cada ejecución de la colección.