use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use dotenvy::dotenv;
use std::env;

mod models;
mod routes;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("La variable DATABASE_URL est introuvable");

    let _pool = db::init_pool(&db_url).await.expect("Échec de la connexion à la base de données");
    println!("Connecté à SQLite !");

    let app = Router::new()
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Serveur visuel démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}