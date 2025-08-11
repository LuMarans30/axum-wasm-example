use axum::{
    Router,
    routing::{get, get_service},
};
use color_eyre::Result;

use crate::view::root::root;
use tower_http::services::ServeDir;
mod view;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/assets", get_service(ServeDir::new("pkg")));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
