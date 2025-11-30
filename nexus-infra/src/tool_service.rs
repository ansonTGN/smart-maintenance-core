use nexus_core::ports::{ToolExecutor, GraphRepository};
use nexus_core::domain::{ToolDefinition, ExecutionResult};
use nexus_core::error::CoreResult;
use std::sync::Arc;
use serde_json::json;

pub struct ToolOrchestrator {
    // Inyectamos el repositorio para herramientas tipo "cypher"
    graph_repo: Arc<dyn GraphRepository>,
    // Cliente HTTP para herramientas tipo "http_api"
    http_client: reqwest::Client, 
}

impl ToolOrchestrator {
    pub fn new(repo: Arc<dyn GraphRepository>) -> Self {
        Self { 
            graph_repo: repo,
            http_client: reqwest::Client::new() 
        }
    }

    pub async fn execute_tool(&self, tool: &ToolDefinition, param: Option<String>) -> CoreResult<ExecutionResult> {
        let data = match tool.r#type.as_str() {
            "cypher" => {
                let q = tool.cypher.as_deref().unwrap_or("");
                let p_val = json!({ "p": param.unwrap_or_default() });
                serde_json::to_value(self.graph_repo.execute_cypher(q, p_val).await?)?
            },
            "http_api" | "wikipedia" => {
                // Aquí iría la lógica HTTP movida desde el main.rs original
                // usando self.http_client
                json!({"status": "executed external api"})
            },
            _ => json!({"error": "Unknown tool type"})
        };

        Ok(ExecutionResult {
            tool_id: tool.id.clone(),
            data,
            is_graph: tool.is_graph
        })
    }
}