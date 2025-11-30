use nexus_infra::services::tool_service::ToolOrchestrator;
use nexus_core::domain::ToolDefinition;
use std::sync::Arc;

pub struct AppState {
    pub tool_service: Arc<ToolOrchestrator>,
    pub tools_config: Vec<ToolDefinition>,
    pub tera: tera::Tera,
}