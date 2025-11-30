use actix_web::{web, App, HttpServer};
use nexus_infra::neo4j_repo::Neo4jAdapter;
use nexus_infra::services::tool_service::ToolOrchestrator;
use nexus_infra::config::load_tools;
use nexus_core::ports::GraphRepository;
use std::sync::Arc;
use dotenv::dotenv;
use tera::Tera;

mod handlers;
mod state;
mod ui; 

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // 1. Cargar Configuración
    let tools = load_tools("queries.json").expect("❌ Error: queries.json no encontrado");
    let uri = std::env::var("NEO4J_URI").expect("NEO4J_URI missing");
    let user = std::env::var("NEO4J_USERNAME").expect("NEO4J_USERNAME missing");
    let pass = std::env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD missing");

    // 2. Infraestructura
    println!("🔌 Conectando a Neo4j...");
    let graph_repo: Arc<dyn GraphRepository> = Arc::new(
        Neo4jAdapter::new(&uri, &user, &pass).await.expect("Failed to connect to Neo4j")
    );
    let tool_service = Arc::new(ToolOrchestrator::new(graph_repo.clone()));

    // 3. Plantillas
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(_) => Tera::new("nexus-app/templates/**/*").expect("❌ Plantillas no encontradas"),
    };

    let app_data = web::Data::new(state::AppState {
        tool_service,
        tools_config: tools,
        tera: tera,
    });

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            // API
            .route("/api/execute", web::post().to(handlers::execute_tool))
            .route("/api/search", web::get().to(handlers::search_nodes)) // <--- AQUÍ ESTÁ LA MAGIA
            
            // UI
            .route("/", web::get().to(ui::index))
            .route("/dashboard", web::get().to(ui::dashboard))
            .route("/connect", web::post().to(ui::dashboard)) 
            .route("/query", web::post().to(ui::execute_query))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}