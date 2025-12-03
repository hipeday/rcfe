mod index;

use axum::{Router, routing::get};
use example_core::{DefaultAppState, get_endpoints, init};
use rcfe::{Client, ClientFactory, ClientOptions, DefaultClientFactory};
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), rcfe::Error> {
    init();

    let app = app().await;

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 5266))
        .await
        .unwrap();

    info!("Starting key-value example on http://localhost:5266");

    axum::serve(listener, app?).await.unwrap();

    Ok(())
}

async fn app() -> Result<Router, rcfe::Error> {
    let endpoints = get_endpoints();

    let client = DefaultClientFactory::new()
        .create(ClientOptions::builder().endpoints(endpoints).build())
        .await?;

    client.ping().await?;

    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(index::page))
        .route("/{lang}", get(index::page_with_lang))
        .with_state(DefaultAppState::new(client)))
}
