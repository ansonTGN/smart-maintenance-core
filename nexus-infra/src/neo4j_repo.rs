use nexus_core::ports::GraphRepository;
use nexus_core::error::{CoreError, CoreResult};
use nexus_core::domain::SearchResult; // <--- Importar
use neo4rs::{Graph, ConfigBuilder, query, BoltType};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

pub struct Neo4jAdapter {
    graph: Arc<Graph>,
}

impl Neo4jAdapter {
    pub async fn new(uri: &str, user: &str, pass: &str) -> Result<Self, anyhow::Error> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(pass)
            .max_connections(10)
            .build()?;
        let graph = Graph::connect(config).await?;
        Ok(Self { graph: Arc::new(graph) })
    }
}

// Helper para conversión de tipos
fn json_to_bolt(v: Value) -> BoltType {
    match v {
        Value::String(s) => s.into(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() { i.into() } 
            else if let Some(f) = n.as_f64() { f.into() } 
            else { 0.into() }
        },
        Value::Bool(b) => b.into(),
        Value::Null => "".into(),
        _ => v.to_string().into(),
    }
}

#[async_trait]
impl GraphRepository for Neo4jAdapter {
    async fn execute_cypher(&self, cypher: &str, params: Value) -> CoreResult<Vec<Value>> {
        let mut q = query(cypher);
        if let Some(obj) = params.as_object() {
            for (k, v) in obj {
                q = q.param(k.as_str(), json_to_bolt(v.clone()));
            }
        }
        let mut stream = self.graph.execute(q).await
            .map_err(|e| CoreError::Infrastructure(e.to_string()))?;

        let mut results = Vec::new();
        while let Ok(Some(row)) = stream.next().await {
            if let Ok(val) = row.to::<Value>() { results.push(val); }
        }
        Ok(results)
    }

    // --- IMPLEMENTACIÓN DEL BUSCADOR ---
    async fn search_nodes(&self, term: &str) -> CoreResult<Vec<SearchResult>> {
        // Buscamos nodos que coincidan en ID o Nombre (case insensitive)
        let cypher = r#"
            MATCH (n) 
            WHERE (n:Material OR n:Equipo OR n:UbicacionTecnica) 
              AND (toLower(n.id) CONTAINS toLower($q) OR toLower(n.name) CONTAINS toLower($q))
            RETURN n.id as id, n.name as name, labels(n)[0] as label 
            LIMIT 20
        "#;

        let q = query(cypher).param("q", term);
        let mut stream = self.graph.execute(q).await
            .map_err(|e| CoreError::Infrastructure(format!("Search Error: {}", e)))?;

        let mut results = Vec::new();
        while let Ok(Some(row)) = stream.next().await {
            let id: String = row.get("id").unwrap_or_default();
            let name: String = row.get("name").unwrap_or_default();
            let label: String = row.get("label").unwrap_or_default();

            results.push(SearchResult {
                id: id.clone(),
                title: format!("{} - {}", id, name),
                label,
            });
        }
        Ok(results)
    }
    // -----------------------------------

    async fn find_node_by_id(&self, _id: &str) -> CoreResult<Option<String>> {
        Ok(None) 
    }
}