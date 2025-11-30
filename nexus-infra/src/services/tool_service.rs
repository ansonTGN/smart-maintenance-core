use nexus_core::ports::GraphRepository;
use nexus_core::domain::{ToolDefinition, ExecutionResult, SearchResult};
use nexus_core::error::CoreResult;
use crate::http_client::HttpAdapter;
use std::sync::Arc;
use serde_json::{json, Value};

pub struct ToolOrchestrator {
    graph_repo: Arc<dyn GraphRepository>,
    http_adapter: HttpAdapter,
}

impl ToolOrchestrator {
    pub fn new(repo: Arc<dyn GraphRepository>) -> Self {
        Self { 
            graph_repo: repo,
            http_adapter: HttpAdapter::new(),
        }
    }

    pub async fn execute_tool(&self, tool: &ToolDefinition, param: Option<String>) -> CoreResult<ExecutionResult> {
        let p_str = param.unwrap_or_default();
        
        let data = match tool.r#type.as_str() {
            "cypher" => {
                let q = tool.cypher.as_deref().unwrap_or("");
                let params = json!({ "p": p_str });
                let rows = self.graph_repo.execute_cypher(q, params).await?;
                serde_json::to_value(rows).unwrap_or(Value::Null)
            },
            "wikipedia" => {
                self.http_adapter.execute_wikipedia(&p_str).await?
            },
            "http_api" => {
                self.http_adapter.execute_generic_api(tool, &p_str).await?
            },
            _ => json!({ "error": "Tipo de herramienta no soportado" })
        };

        Ok(ExecutionResult {
            tool_id: tool.id.clone(),
            title: tool.title.clone(),
            description: tool.description.clone(),
            data,
            is_graph: tool.is_graph
        })
    }

    // --- NUEVO MÉTODO DE BÚSQUEDA ---
    pub async fn search(&self, term: String) -> CoreResult<Vec<SearchResult>> {
        self.graph_repo.search_nodes(&term).await
    }
}