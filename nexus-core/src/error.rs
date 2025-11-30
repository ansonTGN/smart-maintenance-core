use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Error de Infraestructura (DB/Network): {0}")]
    Infrastructure(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Entrada inválida: {0}")]
    InvalidInput(String),

    #[error("Error de configuración: {0}")]
    ConfigError(String),
    
    #[error("Error desconocido: {0}")]
    Unknown(String),
}

// Alias conveniente para Result
pub type CoreResult<T> = Result<T, CoreError>;