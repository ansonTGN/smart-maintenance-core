use actix_web::{web, HttpResponse, Responder, ResponseError};
use crate::state::AppState;
use nexus_core::error::CoreError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

// 1. EL WRAPPER (Patrón New Type)
#[derive(Debug)]
pub struct AppError(CoreError);

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match &self.0 {
            CoreError::NotFound(msg) => HttpResponse::NotFound().json(json!({"error": msg})),
            CoreError::InvalidInput(msg) => HttpResponse::BadRequest().json(json!({"error": msg})),
            CoreError::Infrastructure(msg) => HttpResponse::BadGateway().json(json!({"error": msg})),
            CoreError::ConfigError(msg) => HttpResponse::InternalServerError().json(json!({"error": msg})),
            CoreError::Unknown(msg) => HttpResponse::InternalServerError().json(json!({"error": msg})),
        }
    }
}

impl From<CoreError> for AppError {
    fn from(error: CoreError) -> Self {
        AppError(error)
    }
}

// --- HANDLERS ---

#[derive(Deserialize)]
pub struct ExecuteParams {
    pub query_id: String,
    pub param: Option<String>,
}

pub async fn execute_tool(
    data: web::Data<AppState>,
    body: web::Json<ExecuteParams>
) -> Result<impl Responder, AppError> {
    let tool_def = data.tools_config.iter()
        .find(|t| t.id == body.query_id)
        .ok_or_else(|| AppError(CoreError::NotFound("Tool ID not found".to_string())))?;

    let result = data.tool_service.execute_tool(tool_def, body.param.clone()).await?;
    Ok(HttpResponse::Ok().json(result))
}

// --- NUEVO HANDLER DE BÚSQUEDA ---
#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

pub async fn search_nodes(
    data: web::Data<AppState>,
    info: web::Query<SearchParams>
) -> Result<impl Responder, AppError> {
    let term = info.q.clone().unwrap_or_default();
    let results = data.tool_service.search(term).await?;
    Ok(HttpResponse::Ok().json(results))
}