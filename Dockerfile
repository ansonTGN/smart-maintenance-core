# =========================================================
# ETAPA 1: BUILDER
# =========================================================
FROM rust:1-bookworm AS builder

WORKDIR /app

# 1. Copiamos los manifiestos
COPY Cargo.toml Cargo.lock ./
COPY nexus-core/Cargo.toml nexus-core/
COPY nexus-infra/Cargo.toml nexus-infra/
COPY nexus-app/Cargo.toml nexus-app/

# 2. Creamos código dummy
RUN mkdir -p nexus-core/src && touch nexus-core/src/lib.rs && \
    mkdir -p nexus-infra/src && touch nexus-infra/src/lib.rs && \
    mkdir -p nexus-app/src && echo "fn main() {}" > nexus-app/src/main.rs

# 3. Compilamos dependencias (Capa pesada cacheada)
RUN cargo build --release

# 4. Borramos código dummy
RUN rm -rf nexus-core/src nexus-infra/src nexus-app/src

# 5. Copiamos código real
COPY . .

# 6. CRUCIAL: Forzamos el "touch" en TODOS los miembros del workspace.
#    Esto asegura que Cargo sepa que las librerías han cambiado y no use la versión vacía cacheada.
RUN touch nexus-core/src/lib.rs && \
    touch nexus-infra/src/lib.rs && \
    touch nexus-app/src/main.rs

# 7. Compilamos el binario final
RUN cargo build --release --bin nexus-app


# =========================================================
# ETAPA 2: RUNNER
# =========================================================
FROM debian:bookworm-slim AS runner

WORKDIR /app

# Instalar SSL y certificados
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

ENV RUST_LOG="info" \
    PORT="8080" \
    TZ="Etc/UTC"

# Copiamos binario y recursos
COPY --from=builder /app/target/release/nexus-app /usr/local/bin/
COPY queries.json ./
COPY templates ./templates

EXPOSE 8080

CMD ["nexus-app"]