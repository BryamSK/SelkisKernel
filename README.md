# K-Selkis — Exokernel nativo para WebAssembly

Selkis es un sistema operativo experimental basado en un exokernel, escrito principalmente en Rust, diseñado desde cero para ser WASM-first y WASI-native.

Su objetivo es proporcionar un sustrato mínimo, seguro y eficiente para ejecutar aplicaciones WebAssembly directamente sobre el hardware, sin depender de un kernel POSIX tradicional.

## Propósito

Selkis nace para explorar una pregunta fundamental:

¿Cómo sería un sistema operativo moderno si lo diseñáramos hoy,
pensando primero en cloud, aislamiento fuerte y runtimes,
en lugar de procesos POSIX y compatibilidad histórica?

En K-Selkis:

    El kernel solo gestiona recursos
    Las abstracciones viven en runtimes y Library OS
    WebAssembly es el formato ejecutable principal
    WASI es el contrato estándar de interacción

## Filosofía de diseño

Selkis se guía por los siguientes principios:
El kernel no impone políticas de alto nivel. Solo:

    Asigna CPU
    Gestiona memoria física
    Maneja interrupciones
    Expone dispositivos
    Hace cumplir capabilities
Todo lo demás vive en userland.

🔹 WASM-first, no POSIX-first

    No asume POSIX como modelo base
    No implementa fork, exec, señales ni syscalls Unix
    WASI es la interfaz primaria
    POSIX puede existir solo como LibOS opcional

🔹 Seguridad por construcción
    
    Rust como lenguaje principal del kernel
    Modelo capability-based
    Aislamiento explícito
    Sin permisos implícitos
    Sin herencia silenciosa de recursos

🔹 Cloud, serverless y edge como ciudadanos de primera clase Selkis está pensado para:

    Cargas efímeras
    Arranques rápidos
    Sandboxing fuerte
    Workloads especializados
    Infraestructura controlada
No es un SO de escritorio, y no intenta serlo.

## 🏗 Arquitectura (visión general)

Hardware
  ↓

UEFI + Limine
  ↓

Selkis Exokernel (Rust)
  ↓

WASM Runtime (WASI)
  ↓
  
Aplicaciones WebAssembly

    El kernel no conoce WASI
    El runtime traduce WASI → capabilities
    Las aplicaciones solo ven lo que se les concede

## 🚀 Utilidad práctica

Selkis aspira a ser un entorno ideal para:

    Serverless runtimes
    Edge computing
    Plugins y extensiones seguras
    Infraestructura multi-tenant
    Experimentación en diseño de SO modernos
    Aprendizaje profundo de Rust, kernels y WASM

## 🔭 Visión a futuro

A largo plazo, Selkis busca:

    Ser un host nativo de WebAssembly
    Ejecutar cualquier aplicación compatible con WASI
    Explorar modelos post-POSIX de sistemas operativos
    Servir como base para runtimes especializados
    Demostrar que un exokernel moderno es viable en la era cloud
    Selkis no pretende reemplazar Linux, sino explorar el siguiente paso.

## ⚠ Estado del proyecto
Selkis es un proyecto experimental y educativo.

    No es estable
    No es seguro para producción
    No garantiza compatibilidad con software legacy
    Su valor está en el diseño, la investigación y el aprendizaje.

## 📜 GNU General Public License v3 (GPL-3.0)
