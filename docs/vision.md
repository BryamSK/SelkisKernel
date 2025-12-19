1. Introducción

La idea principal de este proyecto es vislumbrar un futuro en el que las personas no adquieran un procesador local, sino una terminal remota capaz de ejecutar en tiempo real únicamente aquello que necesitan. El objetivo es reducir los costos de adquisición y operación de los servicios. Hoy en día, compramos PCs o laptops completas con capacidades elevadas solo para ejecutar un sistema operativo base, cuando en realidad la mayoría de las personas únicamente necesita una pantalla desde la cual ejecutar una aplicación o resolver un problema, sin importar dónde se esté ejecutando el procesamiento necesario para satisfacer dicha necesidad.

Actualmente vivimos en un mundo orientado al Cloud Computing, con la expectativa de que la inteligencia artificial resuelva gran parte de los problemas que enfrentamos. Sin embargo, existe un desafío fundamental: reducir el consumo energético mientras se incrementa la capacidad de procesamiento de los servidores. Si asumimos un futuro en el que la nube será el principal medio para almacenar, gestionar y distribuir la información, surge una pregunta clave: ¿es realmente necesario contar con sistemas operativos locales que consuman tantos recursos? ¿Tiene sentido que los dispositivos personales incorporen tanta capacidad de procesamiento, o resulta más viable trasladar el procesamiento intensivo a la nube y que los dispositivos de uso masivo se limiten a mostrar la información con un consumo mínimo de recursos?

Esta idea se inspira en el modelo actual de distribución de energía eléctrica, el cual ha funcionado eficazmente durante décadas. Cada hogar no cuenta con un generador propio, sino que consume la energía necesaria desde una red estable y centralizada. De igual forma, los dispositivos dentro del hogar no generan su propia electricidad, sino que la obtienen de dicha red. De manera análoga, en un mundo donde las telecomunicaciones dependen principalmente de la red global de Internet, ya disponemos de los mecanismos para distribuir información, de servidores capaces de procesar grandes volúmenes de datos en tiempo récord y de terminales interactivas para visualizar esos resultados. En esencia, contamos con todas las bases para un modelo en el que los usuarios no necesiten procesar la información localmente, sino simplemente consumirla ya procesada, que es lo que realmente requieren en la mayoría de los casos.

Actualmente se comercializan dispositivos capaces de realizar tareas computacionalmente muy exigentes; sin embargo, la realidad es que la mayoría de las personas solo necesita abrir un navegador y acceder al contenido o recurso requerido. Desde esta perspectiva, la situación es comparable a que cada individuo comprara varios generadores eléctricos, cada uno capaz de abastecer a varias familias: un despropósito que podría evitarse cambiando el enfoque. Es necesario concentrarnos en desarrollar y mejorar soluciones basadas en modelos Cloud, que permitan sistemas más ligeros, eficientes y universales.

Finalmente, considero que modelos como Exokernel y tecnologías como WebAssembly pueden contribuir significativamente a mejorar, desde sus cimientos, la estructura actual de los modelos informáticos, incrementando la eficiencia, portabilidad y sostenibilidad de las aplicaciones existentes.

Exokernel + WebAssembly: una nueva base para sistemas operativos eficientes

Los sistemas operativos actuales (Windows, Linux, macOS) siguen mayoritariamente un modelo monolítico o de kernel híbrido, donde el sistema operativo abstrae el hardware mediante capas complejas: planificadores, sistemas de archivos, controladores, llamadas al sistema y entornos de ejecución. Aunque este enfoque ha demostrado ser robusto y flexible, introduce una sobrecarga significativa en términos de consumo de recursos, latencia y complejidad.

Un sistema operativo basado en un modelo Exokernel, combinado con WebAssembly (Wasm) como formato de ejecución, propone una ruptura fundamental con este paradigma.

1. Exokernel: eliminación de abstracciones innecesarias

El principio central del Exokernel es no imponer abstracciones de alto nivel sobre el hardware. En lugar de ofrecer servicios predefinidos (como sistemas de archivos, procesos o sockets), el Exokernel:

Expone los recursos de hardware de forma segura y controlada.

Permite que las aplicaciones o runtimes gestionen directamente CPU, memoria y almacenamiento.

Reduce la cantidad de código que se ejecuta en modo kernel.

Impacto en el rendimiento

Menos cambios de contexto entre usuario y kernel.

Menor latencia en operaciones críticas (I/O, memoria, scheduling).

Mejor aprovechamiento del hardware, al permitir optimizaciones específicas por aplicación.

En comparación, los kernels tradicionales deben atender a millones de casos genéricos, sacrificando eficiencia para mantener compatibilidad y abstracciones universales.

2. WebAssembly como entorno de ejecución universal

WebAssembly es un formato binario portable, compacto y cercano al metal, diseñado originalmente para la web pero hoy ampliamente adoptado en servidores, edge computing y sistemas embebidos.

Ventajas técnicas de WebAssembly

Ejecución casi nativa gracias a compilación AOT/JIT.

Sandbox de seguridad por diseño (memoria lineal, control de accesos).

Arranque ultrarrápido, ideal para workloads efímeros.

Independencia del lenguaje (C, C++, Rust, Go, etc.).

En un sistema operativo Exokernel, Wasm puede actuar como:

Runtime de aplicaciones

Capa de portabilidad entre hardware y software

Sustituto de procesos tradicionales

3. Sinergia Exokernel + WebAssembly

La combinación de Exokernel y WebAssembly crea un modelo donde:

El Exokernel gestiona únicamente la multiplexación y protección del hardware.

Wasm se ejecuta directamente sobre el Exokernel, sin pasar por capas pesadas del sistema operativo.

Cada aplicación define su propio “mini-OS” en espacio de usuario (filesystem, networking, threading).

Comparación con sistemas actuales
Aspecto	SO tradicional	Exokernel + Wasm
Tamaño del kernel	Muy grande	Mínimo
Abstracciones	Impuestas	Definidas por la app
Overhead	Alto	Muy bajo
Seguridad	Basada en permisos	Sandbox + capacidades
Portabilidad	Dependiente del SO	Nativa (Wasm)
4. Mejora en velocidad y eficiencia energética
Menos consumo de CPU

Menos capas → menos instrucciones ejecutadas.

Menos interrupciones y syscalls.

Mejor uso de memoria

Wasm utiliza memoria lineal controlada.

Eliminación de servicios no utilizados del sistema operativo.

Eficiencia energética

Menor actividad del kernel.

Procesamiento más concentrado en servidores o nodos cloud optimizados.

Terminales ligeras con mínima capacidad de cómputo.

Esto resulta clave en entornos cloud, edge y dispositivos masivos, donde cada milisegundo y cada watt importan.

5. Seguridad sin sacrificar rendimiento

A diferencia de los sistemas tradicionales, donde la seguridad se añade como capas adicionales:

Exokernel usa control de capacidades para el acceso a recursos.

WebAssembly proporciona aislamiento fuerte por defecto.

Las fallas se limitan a la aplicación, no comprometen todo el sistema.

Esto reduce la necesidad de mecanismos costosos como antivirus, virtualización completa o contenedores pesados.

6. Implicaciones para el futuro del cómputo

Un sistema operativo basado en Exokernel + WebAssembly permite:

Dispositivos cliente extremadamente simples.

Sistemas operativos casi invisibles para el usuario.

Aplicaciones verdaderamente universales.

Migración transparente entre local, edge y cloud.

Reducción drástica del costo y consumo energético global.

Este modelo redefine el sistema operativo no como un entorno pesado, sino como un hipervisor ultraligero de recursos, alineado con un mundo distribuido, escalable y orientado a servicios.


Qué es Selkis

Qué no intenta ser

Contexto moderno (cloud, WASM, runtimes)

Ejemplo de ideas clave:

Post-POSIX

Exokernel

WASM-first

Seguridad por capacidades

2. Problema que aborda

Explicar claramente:

    Limitaciones del modelo POSIX
    Overhead de kernels monolíticos en cloud
    Contenedores como LibOS implícitas
    Fragmentación actual (Linux + containers + runtimes)

3. Principios fundamentales

    Aquí deben quedar por escrito:
   Exokernel minimalista
    Kernel ≠ políticas
    WASM como formato ejecutable primario
    WASI como ABI
    Seguridad explícita
    Rust como herramienta de diseño, no solo lenguaje

4. Público objetivo

Muy importante aclarar esto:

✔ Cloud
✔ Edge
✔ Serverless
✔ Investigación
✔ Aprendizaje

❌ Desktop
❌ Usuarios finales
❌ Legacy pesado

5. Visión a largo plazo

Host nativo de WASM
Runtimes especializados
POSIX como opción, no requisito
Exploración de diseño de SO moderno