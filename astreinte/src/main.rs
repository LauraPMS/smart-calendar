use axum::{
    routing::get,
    Router,
};

// declaration des modules

mod models;
mod routes;
mod db;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(racine));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
        
    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn racine() -> &'static str {
    "API Astreintes SSI - Serveur Opérationnel !"
}