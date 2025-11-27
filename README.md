# Rust Microservice Template

Este proyecto es una plantilla profesional, robusta y escalable para desarrollar microservicios en Rust, diseñada bajo los principios de **Arquitectura Hexagonal (Clean Architecture)**.

Su objetivo es estandarizar el desarrollo, facilitar el mantenimiento y asegurar que la lógica de negocio permanezca desacoplada de la infraestructura.

## 🏗 Arquitectura

El proyecto sigue una estricta separación de responsabilidades:

### 1. Domain (`src/domain`)

Es el **núcleo** del software. Aquí residen las reglas de negocio, las estructuras de datos (Entidades) y las definiciones de contratos (Puertos/Traits).

- **Regla de Oro**: No debe depender de ninguna otra capa (ni `infrastructure` ni `implementation`).

### 2. Implementation (`src/implementation`)

Contiene la **Lógica de Negocio** (Casos de Uso). Aquí se definen los "Servicios" que orquestan las operaciones.

- Usa los objetos del `Domain`.
- Usa los `Ports` definidos en el `Domain` para comunicarse con el mundo exterior (BD, APIs), sin saber _cómo_ están implementados.

### 3. Infrastructure (`src/infrastructure`)

Contiene los **Detalles Técnicos**. Aquí es donde la "magia" real ocurre:

- **Persistence**: Implementación de repositorios (MongoDB, Postgres, etc.).
- **HTTP**: Controladores (Handlers) de la API REST, DTOs y configuración de rutas.
- **Providers**: Clientes para servicios externos (Redis, APIs de terceros).

### 4. Composition Root (`src/main.rs`)

Es el punto de entrada. Aquí se cargan las configuraciones, se instancian los repositorios y servicios, y se inyectan las dependencias en el `AppContext`.

---

## 🚀 Inicio Rápido

### Prerrequisitos

- [Rust](https://www.rust-lang.org/tools/install) (Stable)
- Docker & Docker Compose (opcional, para levantar MongoDB localmente)

### Configuración

1.  **Variables de Entorno**:
    Crea un archivo `.env` en la raíz del proyecto basado en las necesidades definidades en `src/envs.rs`.

    ```bash
    PORT=8080
    SERVICE_NAME=my-service
    MONGO_URL=mongodb://localhost:27017
    MONGO_DB=my_database
    DEBUG_LEVEL=info
    ```

2.  **Base de Datos**:
    Asegúrate de tener una instancia de MongoDB corriendo.

### Ejecución

Modo desarrollo (recomendado usar `cargo-watch`):

```bash
cargo watch -x run
```

Ejecución estándar:

```bash
cargo run
```

---

## 🛠 Guía de Desarrollo

Para agregar una nueva funcionalidad (ej. "Usuarios"), sigue este flujo estricto para mantener la arquitectura limpia:

### Paso 1: Domain

Define **qué** es un usuario y **qué** necesitamos hacer con él.

- Crear `src/domain/entities/user.rs` (Struct `User`).
- Crear `src/domain/ports/user_port.rs` (Trait `PortUserRepo`).

### Paso 2: Implementation

Implementa la lógica de negocio.

- Crear `src/implementation/user_service.rs`.
- El servicio debe tener un campo `repo: Arc<dyn PortUserRepo>`.
- Implementa métodos como `create_user`, `find_user`. **Documenta** estos métodos con `///`.

### Paso 3: Infrastructure

Implementa los detalles técnicos.

- **Persistencia**: Crear `src/infrastructure/persistence/user_repo.rs`. Implementa el trait `PortUserRepo` usando MongoDB.
- **HTTP**:
  - Definir DTOs en `src/infrastructure/http/dto/user_dto.rs` (Request/Response).
  - Crear Handlers en `src/infrastructure/http/handlers/user_handler.rs`.

### Paso 4: Wiring (Conexión)

Conecta todo en el arranque.

1.  En `src/ctx.rs`: Agrega `user_srv: UserService` al struct `AppContext`.
2.  En `src/main.rs`: Instancia el repositorio, luego el servicio, e inyectalo en el contexto.
3.  En `src/infrastructure/http/routes.rs`: Registra las nuevas rutas.

---

## ✅ Health Check

El servicio incluye un endpoint de salud robusto que verifica la conexión a la base de datos:

```bash
curl http://localhost:8080/healthz
```

- **200 OK**: El servicio y la base de datos funcionan correctamente.
- **503 Service Unavailable**: El servicio funciona, pero la conexión a la BD falló.
