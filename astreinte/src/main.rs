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

    // On récupère le pool de connexion
    let pool = db::init_pool(&db_url).await.expect("Échec de la connexion à la base de données");
    println!("Connecté à SQLite !");

    // Configuration des routes
    let app = Router::new()
        .route("/api/users", post(routes::create_user))
        .route("/api/login", post(routes::login_user))
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}