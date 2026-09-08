mod db;
mod handlers;
mod structs;

use dotenvy::dotenv;
use handlers::routes::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app = app().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await?;

    Ok(())
}