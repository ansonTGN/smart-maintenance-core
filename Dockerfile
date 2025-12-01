# =========================================================
# ETAPA 1: BUILDER
# =========================================================
# CAMBIO IMPORTANTE: Usamos '1-bookworm' para obtener la última versión estable de Rust.
# Si prefieres fijar versión, usa al menos rust:1.80-bookworm
FROM rust:1-bookworm AS builder

WORKDIR /app

# 1. Copiamos los manifiestos del ROOT y de TODOS los miembros del workspace
COPY Cargo.toml Cargo.lock ./
COPY nexus-core/Cargo.toml nexus-core/
COPY nexus-infra/Cargo.toml nexus-infra/
COPY nexus-app/Cargo.toml nexus-app/

# 2. Creamos código dummy para CADA miembro para poder compilar las dependencias.
RUN mkdir -p nexus-core/src && touch nexus-core/src/lib.rs && \
    mkdir -p nexus-infra/src && touch nexus-infra/src/lib.rs && \
    mkdir -p nexus-app/src && echo "fn main() {}" > nexus-app/src/main.rs

# 3. Compilamos las dependencias (capa cacheada)
RUN cargo build --release

# 4. Borramos el código dummy para copiar el real
RUN rm -rf nexus-core/src nexus-infra/src nexus-app/src

# 5. Copiamos el código fuente real
COPY . .

# 6. Forzamos a Cargo a notar el cambio en el archivo principal
RUN touch nexus-app/src/main.rs

# 7. Compilamos el binario final
RUN cargo build --release --bin nexus-app


# =========================================================
# ETAPA 2: RUNNER
# =========================================================
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Instalar dependencias de sistema necesarias (SSL/TLS)
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Variables de entorno
ENV RUST_LOG="info" \
    PORT="8080" \
    TZ="Etc/UTC"

# 1. Copiar el binario desde el builder
COPY --from=builder /app/target/release/nexus-app /usr/local/bin/

# 2. Copiar los archivos estáticos requeridos (plantillas y queries)
COPY queries.json ./
COPY templates ./templates

# Expone el puerto
EXPOSE 8080

# Ejecutar
CMD ["nexus-app"]