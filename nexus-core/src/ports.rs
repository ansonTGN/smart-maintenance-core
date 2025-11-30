use async_trait::async_trait;
use serde_json::Value;
use crate::domain::{ToolDefinition, ExecutionResult, SearchResult};
use crate::error::CoreResult;

/// Puerto para persistencia de grafos (Database agnostic)
#[async_trait]
pub trait GraphRepository: Send + Sync {
    async fn execute_cypher(&self, query: &str, params: Value) -> CoreResult<Vec<Value>>;
    
    // --- NUEVO MÉTODO ---
    async fn search_nodes(&self, term: &str) -> CoreResult<Vec<SearchResult>>;
    // --------------------

    async fn find_node_by_id(&self, id: &str) -> CoreResult<Option<String>>;
}

/// Puerto para ejecución de herramientas externas (Strategy Pattern)
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    fn can_execute(&self, tool_type: &str) -> bool;
    async fn execute(&self, tool: &ToolDefinition, param: Option<String>) -> CoreResult<ExecutionResult>;
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn chat(&self, history: Vec<Value>, tools: &[ToolDefinition]) -> CoreResult<Value>;
}