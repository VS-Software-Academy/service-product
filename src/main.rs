pub mod app;
pub mod controller;
pub mod model;
pub mod repository;
pub mod service;
pub mod util;

use crate::{
    app::app_state::AppState,
    controller::{
        category::{categories_create, categories_delete, categories_index, categories_read},
        product::{
            products_create, products_delete, products_index, products_read, products_update,
        },
    },
    repository::{category::DbCategoryRepository, product::DbProductRepository},
};
use axum::{http::StatusCode, routing::get, Router};
use clap::Parser;
use service::{category::CategoryService, product::ProductService};
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, net::SocketAddr};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Project to demonstrate product API service", long_about = None)]
struct Args {
    /// Socket address to start the web server
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Load env variables from .env file
    dotenvy::dotenv()?;

    // Configure trace
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "service_product=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Setup connection pool
    let db_connection_str = std::env::var("DATABASE_URL").expect("database env not defined");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("can connect to database");

    // Create app state
    let app_state = AppState::new(
        ProductService::new(DbProductRepository::new(pool.clone())),
        CategoryService::new(DbCategoryRepository::new(pool)),
    );

    // Define routes
    let app = Router::new()
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

    tracing::debug!("listening on {}", args.addr);
    axum::Server::bind(&args.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
