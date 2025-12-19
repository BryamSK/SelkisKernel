Un diagrama lógico

Hardware
  ↓
UEFI + Limine
  ↓
Selkis Exokernel (Rust)
  ↓
WASM Runtime (WASI)
  ↓
Aplicaciones WebAssembly


Nota:
    El kernel no implementa WASI
    El runtime traduce WASI → capabilities

## 2. Decisiones técnicas clave
Aquí dejamos constancia oficial:

    Arquitectura: x86_64
    Emulación: QEMU
    Firmware: UEFI
    Bootloader: Limine
    Lenguaje del kernel: Rust (no_std)
    Licencia: GPLv3

## 3. Responsabilidades del kernel

Lista explícita de lo que sí hace el kernel:

    ✔ Gestión de CPU
    ✔ Gestión de memoria física
    ✔ Interrupciones
    ✔ Dispositivos
    ✔ Capabilities
    ✔ Aislamiento

No hace:

    ❌ POSIX
    ❌ FS global
    ❌ Networking stack
    ❌ Procesos Unix
    ❌ Señales

4. Modelo de capabilities

Explicar conceptualmente (Pendiente):

    Capabilities como objetos
    Sin permisos implícitos
    Asignación explícita
    Revocación visible

5. WASM / WASI como userland

    WASM = formato ejecutable
    WASI = ABI
    Runtime como boundary
    Apps nunca hablan directo con el kernel

6. Seguridad

    Aislamiento fuerte
    Menor TCB
    Rust + ownership
    Sandbox por diseño