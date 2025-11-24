# Prompt de Desarrollo y Arquitectura - Rust Template

Este documento sirve como contexto maestro y guía de estilo para el desarrollo de funcionalidades en este proyecto. Úsalo como referencia obligatoria al solicitar o generar código.

## 1. Arquitectura del Proyecto

El proyecto implementa una **Arquitectura Hexagonal (Clean Architecture)**. La prioridad es mantener la lógica de negocio independiente de frameworks, bases de datos y herramientas externas.

### Estructura de Directorios

- **`src/domain`**: **Núcleo del sistema.** Contiene Entidades y Puertos (Traits).
  - _Regla_: No debe depender de `infrastructure` ni `implementation`. Código puro de Rust.
- **`src/implementation`**: **Lógica de Negocio.** Contiene los Servicios.
  - _Regla_: Implementa los casos de uso utilizando las interfaces definidas en `domain`.
- **`src/infrastructure`**: **Detalles Técnicos.** Contiene la implementación de los puertos.
  - _Subcarpetas_: `http` (API Server), `persistence` (Base de datos), `providers` (Servicios externos).
- **`src/main.rs`**: **Composition Root.** Configura e inyecta las dependencias.

## 2. Estándares de Programación

### Principios No Negociables

1.  **Legibilidad ante todo**: El código debe ser fácil de leer para cualquier desarrollador. Evita la sobre-ingeniería.
2.  **Mantenibilidad**: Estructura el código pensando en que será modificado en el futuro.
3.  **Idiomático**: Sigue las convenciones estándar de Rust (fmt, clippy).

### Documentación

- **Obligatorio**: Todas las funciones públicas en los servicios (`src/implementation`) deben tener documentación (`///`).
- **Estilo**: **Minimalista**. Usa la menor cantidad de palabras posibles para describir el propósito.

_Ejemplo Correcto:_

```rust
/// Valida y registra un nuevo pedido.
pub async fn crear_pedido(...)
```

_Ejemplo Incorrecto:_

```rust
/// Esta función se encarga de recibir los datos del pedido, validar que el stock exista,
/// llamar al repositorio para guardar en base de datos y luego retornar el resultado.
pub async fn crear_pedido(...)
```

## 3. Flujo de Desarrollo para Nuevas Features

Para añadir una nueva característica, sigue estrictamente este orden:

1.  **Domain**: Define las `Entities` y los `Ports` (traits de repositorios/servicios) en `src/domain`.
2.  **Implementation**: Crea el `Service` que orquesta la lógica en `src/implementation`.
3.  **Infrastructure**: Implementa los puertos (ej. Repositorio Mongo) y los Handlers HTTP en `src/infrastructure`.
4.  **Wiring**: Instancia y conecta las dependencias en `src/main.rs` y actualiza `src/ctx.rs`.

---

**Nota para IA:** Al generar código, verifica siempre `src/main.rs` para entender cómo se inyectan las dependencias actuales y replica ese patrón.
