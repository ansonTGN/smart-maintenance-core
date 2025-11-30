use nexus_core::error::{CoreError, CoreResult};
use nexus_core::domain::ToolDefinition;
use serde_json::Value;

pub struct HttpAdapter {
    client: reqwest::Client,
}

impl HttpAdapter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Nexus-Industrial-Dashboard/2.0")
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn execute_wikipedia(&self, param: &str) -> CoreResult<Value> {
        let url = "https://es.wikipedia.org/w/api.php";
        let params = [
            ("action", "query"),
            ("format", "json"),
            ("list", "search"),
            ("srsearch", param),
            ("utf8", "1"),
            ("srlimit", "3"),
        ];

        let res = self.client.get(url).query(&params).send().await
            .map_err(|e| CoreError::Infrastructure(format!("Wikipedia Error: {}", e)))?;

        let json: Value = res.json().await
            .map_err(|e| CoreError::Infrastructure(format!("JSON Parse Error: {}", e)))?;

        // Extraer solo la parte relevante
        Ok(json.get("query").and_then(|q| q.get("search")).cloned().unwrap_or(json))
    }

    pub async fn execute_generic_api(&self, tool: &ToolDefinition, param: &str) -> CoreResult<Value> {
        let url_template = tool.api_url.as_deref().ok_or(CoreError::ConfigError("URL faltante".into()))?;
        let final_url = url_template.replace("{p}", param);
        let method = tool.method.as_deref().unwrap_or("GET").to_uppercase();

        let req = match method.as_str() {
            "POST" => self.client.post(&final_url),
            "PUT"  => self.client.put(&final_url),
            _      => self.client.get(&final_url),
        };

        let res = req.send().await
            .map_err(|e| CoreError::Infrastructure(format!("HTTP Error: {}", e)))?;

        res.json::<Value>().await
            .map_err(|_| CoreError::Infrastructure("Respuesta no es JSON válido".into()))
    }
}