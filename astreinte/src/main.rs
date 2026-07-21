use axum::{routing::{get, post, delete}, Router};
use std::env;
use tower_http::services::ServeDir;

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL non trouvée");
    let pool = db::init_pool(&db_url).await.expect("Échec connexion BDD");

    let app = Router::new()
        // Auth
        .route("/api/login", post(routes::auth::login_user))
        .route("/api/change-password", post(routes::auth::change_password))     
        // Admin
        .route("/api/admin/services", post(routes::admin::create_service).get(routes::admin::get_services))
        .route("/api/admin/users", post(routes::admin::create_user).get(routes::admin::get_users))        
        .route("/api/admin/users/{user_id}", delete(routes::admin::delete_user))
        // Shifts
        .route("/api/shifts", post(routes::user::create_shift_request))
        .route("/api/shifts/{service_id}", get(routes::user::get_service_shifts))
        .route("/api/manager/shifts/update", post(routes::manager::update_shift_status))
        // Static Files
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur lancé sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}