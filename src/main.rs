pub mod app;
pub mod controllers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

use crate::{
    app::app_state::AppState,
    controllers::{
        category::{categories_create, categories_delete, categories_index, categories_read},
        product::{
            products_create, products_delete, products_index, products_read, products_update,
        },
    },
    repositories::{category::DbCategoryRepository, product::DbProductRepository},
};
use axum::{
    http::{HeaderValue, StatusCode},
    routing::get,
    Router,
};
use clap::Parser;
use services::{category::CategoryService, product::ProductService};
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{AllowHeaders, AllowMethods, CorsLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Project to demonstrate product API service", long_about = None)]
struct Args {
    /// Socket address to start the web server
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    addr: SocketAddr,
    /// Enable allow cors origin
    #[arg(short, long)]
    cors_allow_origin: Option<HeaderValue>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Configure trace
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "service_product=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load env variables from .env file
    if let Err(e) = dotenvy::dotenv() {
        tracing::debug!("attempt to read .env: `{e}`");
    }

    // Setup connection pool
    let db_connection_str = std::env::var("DATABASE_URL").expect("database env not defined");

    tracing::info!("try connecting {db_connection_str}...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // Create app state
    let app_state = AppState::new(
        ProductService::new(DbProductRepository::new(pool.clone())),
        CategoryService::new(DbCategoryRepository::new(pool)),
    );

    // Define routes
    let mut app = Router::new()
        .route("/api/products", get(products_index).post(products_create))
        .route(
            "/api/products/:id",
            get(products_read)
                .patch(products_update)
                .delete(products_delete),
        )
        .route(
            "/api/categories",
            get(categories_index).post(categories_create),
        )
        .route(
            "/api/categories/:id",
            get(categories_read)
                .patch(categories_create)
                .delete(categories_delete),
        )
        .with_state(app_state);

    if let Some(cors_allow_origin) = args.cors_allow_origin {
        tracing::info!("CORS allow origin {cors_allow_origin:?}");
        let cors = CorsLayer::new()
            .allow_headers(AllowHeaders::any())
            .allow_origin(cors_allow_origin)
            .allow_methods(AllowMethods::any());
        app = app.layer(cors);
    }

    tracing::info!("listening on {}", args.addr);
    axum::Server::bind(&args.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
