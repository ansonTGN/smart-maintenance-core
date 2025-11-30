use nexus_core::domain::ToolDefinition;
use nexus_core::error::{CoreError, CoreResult};
use std::fs::File;
use std::io::BufReader;

pub fn load_tools(path: &str) -> CoreResult<Vec<ToolDefinition>> {
    let file = File::open(path)
        .map_err(|e| CoreError::ConfigError(format!("No se pudo abrir {}: {}", path, e)))?;
    
    let reader = BufReader::new(file);
    
    let tools: Vec<ToolDefinition> = serde_json::from_reader(reader)
        .map_err(|e| CoreError::ConfigError(format!("Error parseando JSON: {}", e)))?;
        
    Ok(tools)
}