# =========================================================
# ETAPA 1: BUILDER - Compila la aplicación
# =========================================================
# Usamos una versión específica y reciente de Rust para evitar problemas
# con el formato de Cargo.lock (Versión 4 o superior). 
# AJUSTA esta versión (1.77) si tu versión local de Cargo es mucho más nueva.
FROM rust:1.77 AS builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de manifiesto para que Docker pueda cachear las dependencias
# (Esta es la capa más pesada y la que queremos cachear si solo cambia el código fuente)
COPY Cargo.toml Cargo.lock ./

# Creamos un binario dummy y lo compilamos. El propósito es forzar
# a Cargo a descargar y compilar todas las dependencias y guardarlas en caché.
# Las dependencias solo se recompilan si Cargo.toml o Cargo.lock cambian.
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    cargo build --release

# Limpiamos el binario dummy
RUN rm -rf src

# Copiamos todo el código fuente real
COPY . .

# Compilamos la aplicación final. Usamos --bin para compilar solo el ejecutable 'nexus-app'.
# Esto se hace para que Docker invalide la caché de esta capa si el código fuente cambia.
RUN cargo build --release --bin nexus-app


# =========================================================
# ETAPA 2: RUNNER - Imagen de Producción Mínima
# =========================================================
# Usamos una imagen base muy pequeña (debian-slim) que solo contiene lo necesario
# para ejecutar el binario estáticamente enlazado.
FROM debian:stable-slim AS runner

# Variables de entorno esenciales
# Render inyectará el puerto, pero es útil para pruebas y claridad.
ENV RUST_LOG="info" \
    PORT="8080" \
    TZ="Etc/UTC"

# Copia el binario compilado de la etapa 'builder' al directorio de PATH
# El binario 'nexus-app' está en /app/target/release/nexus-app en la etapa builder.
COPY --from=builder /app/target/release/nexus-app /usr/local/bin/

# Expone el puerto por defecto
EXPOSE 8080

# Comando para ejecutar el binario al iniciar el contenedor
CMD ["/usr/local/bin/nexus-app"]