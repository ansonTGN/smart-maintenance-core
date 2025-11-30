use actix_web::{web, HttpResponse, Responder};
use crate::state::AppState;
use tera::Context;
use std::collections::{BTreeMap, HashMap};
use nexus_core::domain::ToolDefinition;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryForm {
    pub query_id: String,
    pub param: Option<String>,
}

#[derive(Serialize)]
struct HtmlTableResult {
    columns: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    query_title: String, 
    timestamp: String,
    is_graph: bool,
}

pub async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("env_uri", &std::env::var("NEO4J_URI").unwrap_or_default());
    ctx.insert("env_user", &std::env::var("NEO4J_USERNAME").unwrap_or_default());
    ctx.insert("env_pass", &std::env::var("NEO4J_PASSWORD").unwrap_or_default());
    ctx.insert("error", "");

    match data.tera.render("login.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error template: {:?}", e))
    }
}

fn get_dashboard_context(data: &web::Data<AppState>) -> Context {
    let mut ctx = Context::new();
    ctx.insert("db_host", &std::env::var("NEO4J_URI").unwrap_or_default());
    
    let mut cypher_groups: BTreeMap<String, Vec<ToolDefinition>> = BTreeMap::new();
    let mut ai_groups: BTreeMap<String, Vec<ToolDefinition>> = BTreeMap::new();

    for t in &data.tools_config {
        if t.r#type == "cypher" {
            cypher_groups.entry(t.category.clone()).or_default().push(t.clone());
        } else {
            ai_groups.entry(t.category.clone()).or_default().push(t.clone());
        }
    }

    ctx.insert("categorized_queries", &cypher_groups);
    ctx.insert("ai_tools_groups", &ai_groups);
    ctx
}

pub async fn dashboard(data: web::Data<AppState>) -> impl Responder {
    let mut ctx = get_dashboard_context(&data);
    ctx.insert("current_query", "");
    ctx.insert("current_param", "");
    ctx.insert("current_param_label", "");
    ctx.insert("error", "");
    
    match data.tera.render("dashboard.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error template: {:?}", e))
    }
}

pub async fn execute_query(data: web::Data<AppState>, form: web::Form<QueryForm>) -> impl Responder {
    let mut ctx = get_dashboard_context(&data);
    
    ctx.insert("current_query", &form.query_id);
    let current_param = form.param.clone().unwrap_or_default();
    ctx.insert("current_param", &current_param);
    ctx.insert("current_param_label", &current_param);
    ctx.insert("error", "");

    let tool_def = match data.tools_config.iter().find(|t| t.id == form.query_id) {
        Some(t) => t,
        None => return HttpResponse::BadRequest().body("Herramienta no encontrada"),
    };

    match data.tool_service.execute_tool(tool_def, form.param.clone()).await {
        Ok(exec_result) => {
            let mut rows_vec = Vec::new();
            let mut columns = Vec::new();

            if let serde_json::Value::Array(arr) = exec_result.data {
                for item in arr {
                    if let serde_json::Value::Object(map) = item {
                        if columns.is_empty() {
                            let mut keys: Vec<String> = map.keys().cloned().collect();
                            // Ordenar columnas: Poner ID o NAME primero si existen
                            keys.sort_by(|a, b| {
                                let a_score = if a.contains("ID") { 0 } else if a.contains("NAME") { 1 } else { 2 };
                                let b_score = if b.contains("ID") { 0 } else if b.contains("NAME") { 1 } else { 2 };
                                if a_score == b_score { a.cmp(b) } else { a_score.cmp(&b_score) }
                            });
                            columns = keys;
                        }
                        
                        let mut row_display = HashMap::new();
                        for (k, v) in map {
                            // LIMPIEZA DE DATOS PARA EVITAR ROMPER JS
                            let val_str = match v {
                                serde_json::Value::String(s) => s.clone().replace("'", ""), // Quitamos comillas simples
                                serde_json::Value::Null => "-".to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                serde_json::Value::Number(n) => n.to_string(),
                                _ => v.to_string(),
                            };
                            row_display.insert(k, val_str);
                        }
                        rows_vec.push(row_display);
                    }
                }
            }

            let html_results = HtmlTableResult {
                columns,
                rows: rows_vec,
                query_title: tool_def.title.clone(),
                timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
                is_graph: tool_def.is_graph,
            };

            ctx.insert("results", &html_results);
            
            match data.tera.render("dashboard.html", &ctx) {
                Ok(rendered) => HttpResponse::Ok().body(rendered),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error rendering results: {:?}", e))
            }
        },
        Err(e) => {
            ctx.insert("error", &format!("Error ejecutando consulta: {}", e));
            HttpResponse::Ok().body(data.tera.render("dashboard.html", &ctx).unwrap())
        }
    }
}