use errors::app_error::AppError;
use routes::root::load_routes;
use shared::extract_env::extract_env;
use sqlx::postgres::PgPoolOptions;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod adapters;
mod config;
mod controllers;
mod entities;
mod errors;
mod repositories;
mod routes;
mod services;
mod shared;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let database_url = extract_env::<String>("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let app = load_routes();
    let ip_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000));
    log::info!("Application listening on {}", ip_address);

    let listener = tokio::net::TcpListener::bind(ip_address)
        .await
        .map_err(|err| {
            log::error!(
                "error binding service to address due to {}",
                err.to_string()
            );
            AppError::OperationFailed(err.to_string())
        })?;
    axum::serve(listener, app).await.map_err(|err| {
        log::error!("error starting app due to {}", err.to_string());
        AppError::OperationFailed(err.to_string())
    })?;

    Ok(())
}
