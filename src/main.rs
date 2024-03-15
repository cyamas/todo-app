use axum::{routing::{delete, get, post, patch}, Router};
use tower_http::services::ServeDir;
use sqlx::postgres::PgPoolOptions;
use anyhow::Context;
use dotenv::dotenv;
use todo_app::handlers::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host_url = std::env::var("HOST_URL").expect("HOST_URL must be set");
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .context("could not connect to database url")?;
    

    let app = Router::new()
        .nest_service("/client", ServeDir::new("client"))
        .route("/login", get(login))
        .route("/", get(home))
        .route("/todos/add", post(add_todo))
        .route("/todos/complete", post(complete_todo))
        .route("/todos/editform", post(edit_form))
        .route("/todos/edit", patch(edit_todo))
        .route("/todos/delete", delete(delete_todo))
        .route("/todos/revert", patch(revert_todo))
        .route("/todos/activate", patch(activate_todo))
        .route("/todos/deactivate", patch(deactivate_todo))
        .route("/todos/progress", patch(show_progress))
        .route("/todos/hideprogress", patch(hide_progress))
        .route("/todos/deleteprogress", delete(delete_progress))
        .with_state(pool);
        

    let listener = tokio::net::TcpListener::bind(host_url)
    .await
    .context("Error binding TcpListener to specified port")?; // creates a tcp listener named listener and binds it to port 6900
    
    axum::serve(listener, app)
    .await
    .context("Error serving application")?;

    Ok(())
}




