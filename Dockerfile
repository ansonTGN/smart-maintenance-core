# --- ETAPA 1: BUILD ---
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

# Copiamos todo el código fuente del workspace
COPY . .

# Compilamos el binario específico "nexus-app" en modo release
RUN cargo build --release --bin nexus-app

# --- ETAPA 2: RUNTIME ---
FROM debian:bookworm-slim

# Instalamos certificados SSL (necesarios para conectar a Neo4j Aura) y libssl
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiamos el binario compilado desde la etapa anterior
COPY --from=builder /app/target/release/nexus-app .

# Copiamos los archivos de configuración y plantillas necesarios
# Asegúrate de que estos archivos existan en la raíz de tu repo
COPY --from=builder /app/queries.json .
COPY --from=builder /app/templates ./templates

# Render inyecta la variable PORT, pero tu app hardcodea el 8080.
# Exponemos el 8080 para documentación.
EXPOSE 8080

# Comando de inicio
CMD ["./nexus-app"]