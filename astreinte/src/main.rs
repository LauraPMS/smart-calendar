use axum::{
    routing::post,
    Router,
};
use tower_http::services::ServeDir;
use dotenvy::dotenv;
use std::env;

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable DATABASE_URL est introuvable");

    println!("\n========================================");
    println!("🔍 URL DE LA BASE UTILISÉE : {}", db_url);
    println!("========================================\n");

    let pool = db::init_pool(&db_url).await.expect("Échec de la connexion à la base de données");
    println!("Connecté à SQLite !");

let app = Router::new()
        .route("/api/users", post(routes::create_user))
        .route("/api/login", post(routes::login_user))
        .route("/api/change-password", post(routes::change_password))
        .route("/api/requests", post(routes::create_request).get(routes::get_requests))
        .route("/api/requests/update", post(routes::update_request_status)) // <-- AJOUT ICI
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}