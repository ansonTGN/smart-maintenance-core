use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    
    #[serde(default)]
    pub r#type: String, // "cypher", "http_api", "wikipedia"
    
    // UI fields
    pub category: String,
    #[serde(default)]
    pub icon: String,

    pub title: String,
    pub description: String,
    
    pub cypher: Option<String>,
    pub api_url: Option<String>,
    pub method: Option<String>,
    
    pub needs_param: bool,
    pub is_graph: bool,
}

#[derive(Serialize)]
pub struct ExecutionResult {
    pub tool_id: String,
    pub title: String,
    pub description: String,
    pub data: serde_json::Value,
    pub is_graph: bool,
}

// --- NUEVA ESTRUCTURA PARA EL BUSCADOR ---
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub label: String,
}