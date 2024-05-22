use actix_web::{HttpServer, App};

use dotenv::dotenv;

use crate::config::PoolConfig;

mod routes;
mod handlers;
mod models;
mod db;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  HttpServer::new(move || {
    App::new()
      .configure(routes::init_routes)
  }).bind("0.0.0.0:6065").expect("Failed to bind to port 6065")
    .run().await
}