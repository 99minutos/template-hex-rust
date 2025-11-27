# Contexto y Guía de Desarrollo - Rust Template

Este documento define las reglas estrictas, arquitectura y flujo de trabajo para agentes de IA que desarrollen sobre este repositorio. **Sigue estas instrucciones al pie de la letra.**

## 1. Arquitectura Hexagonal (Clean Architecture)

El proyecto está organizado en capas concéntricas. La dependencia solo puede apuntar hacia adentro.

### Estructura de Directorios

- **`src/domain`** (Núcleo)
  - Contiene la lógica pura y definiciones de tipos.
  - **`entities`**: Structs puros de datos (ej. `Example`).
  - **`ports`**: Traits que definen interfaces para repositorios y servicios externos (ej. `PortExampleRepo`).
  - **NO** debe depender de `infrastructure` ni de frameworks web/db específicos (salvo tipos primitivos de `bson`/`chrono` si es estrictamente necesario para serialización interna).

- **`src/implementation`** (Casos de Uso)
  - Contiene la lógica de negocio orquestada.
  - **`services`**: Implementan la lógica utilizando los `ports` definidos en `domain`.
  - Inyectan dependencias a través de sus constructores (ej. `ExampleService::new(repo)`).

- **`src/infrastructure`** (Adaptadores)
  - Implementaciones concretas de los puertos y adaptadores de entrada.
  - **`persistence`**: Implementación de los repositorios (ej. `MongoDbExampleRepository` implementa `PortExampleRepo`).
  - **`http`**:
    - **`handlers`**: Controladores que reciben requests HTTP, llaman a los servicios y retornan respuestas.
    - **`dto`**: Data Transfer Objects para entrada/salida de la API. Convierte entre DTOs y Entidades de Dominio.
    - **`routes`**: Definición de rutas Axum.

- **`src/main.rs` & `src/ctx.rs`** (Composition Root)
  - Aquí se inicializan las conexiones, se instancian los repositorios y servicios, y se inyectan en el `AppContext`.

## 2. Reglas de Desarrollo

### General

1.  **Idioma**: Comentarios y Documentación en **Español**. Nombres de variables, funciones y archivos en **Inglés**.
2.  **Seguridad**: Validar todos los inputs en la capa HTTP (DTOs). No exponer errores internos de BD al cliente (Stack traces).
3.  **Observabilidad**: Usar `#[tracing::instrument(skip_all)]` en métodos de servicios y repositorios.

### Capa de Dominio (`src/domain`)

1.  Define errores de dominio en `mod.rs` o `error.rs` para desacoplar errores de infraestructura.
2.  Las entidades deben mantenerse agnósticas a la base de datos (sin anotaciones de `serde` específicas de mongo si es posible evitarlo, aunque `bson` es tolerado por pragmatismo).

### Capa de Implementación (`src/implementation`)

1.  **Servicios**: Deben ser `structs` que contienen `Arc<dyn Port...>`.
2.  **Documentación**: Es **OBLIGATORIO** documentar cada método público (`pub fn`) de los servicios con `///` en español.
    - _Estilo_: Breve y directo (ej. "Crea un nuevo usuario y envía notificación").

### Capa de Infraestructura (`src/infrastructure`)

1.  **Persistencia**:
    - Mapear errores de drivers (Mongo, Redis) a `DomainError` antes de retornarlos.
    - Los repositorios devuelven `DomainWrapper<T>`.
2.  **HTTP**:
    - **NUNCA** pasar entidades de dominio directamente como JSON en la respuesta. Usar DTOs (`From<Entity> for Dto`).
    - Los handlers deben ser delgados: `Request -> DTO -> Service -> Response`.
    - Usar `validator` en los DTOs de entrada.

## 3. Flujo de Trabajo para Nuevas Features

Para agregar una funcionalidad, el agente debe seguir este orden estricto:

1.  **Definición (Domain)**:
    - Crear/Editar Entidad en `src/domain/entities`.
    - Definir el Trait del repositorio en `src/domain/ports`.

2.  **Lógica (Implementation)**:
    - Crear/Editar el Servicio en `src/implementation`.
    - Implementar la lógica de negocio.
    - **Documentar** los métodos.

3.  **Persistencia (Infrastructure)**:
    - Implementar el Trait del repositorio en `src/infrastructure/persistence`.

4.  **Exposición (Infrastructure/HTTP)**:
    - Crear DTOs en `src/infrastructure/http/dto`.
    - Crear Handler en `src/infrastructure/http/handlers`.
    - Registrar ruta en `src/infrastructure/http/routes.rs`.

5.  **Inyección (Wiring)**:
    - Agregar el servicio a `AppContext` en `src/ctx.rs`.
    - Instanciar repositorio y servicio, luego conectarlos en `src/main.rs`.

## 4. Patrones de Código Comunes

**Inyección de Dependencias:**

```rust
// src/implementation/my_service.rs
pub struct MyService {
    repo: Arc<dyn PortMyRepo>,
}

impl MyService {
    pub fn new(repo: Arc<dyn PortMyRepo>) -> Self {
        Self { repo }
    }
}
```

**Manejo de Errores (Repository):**

```rust
// src/infrastructure/persistence/my_repo.rs
match self.db.find_one(...).await {
    Ok(Some(doc)) => Ok(doc),
    Ok(None) => Err(DomainError::new(ErrorKind::NotFound, "Elemento no encontrado")),
    Err(e) => Err(DomainError::new(ErrorKind::Database, format!("Error DB: {}", e))),
}
```

**Handler Axum:**

```rust
pub async fn create_item(
    State(ctx): State<Arc<AppContext>>,
    Json(req): Json<CreateItemDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // 1. Validar DTO (si aplica)
    req.validate().map_err(|e| ...)?;

    // 2. Llamar servicio
    let entity = ctx.my_service.create(req.into_domain()).await?;

    // 3. Responder con DTO de salida
    Ok((StatusCode::CREATED, Json(ItemDto::from(entity))))
}
```
