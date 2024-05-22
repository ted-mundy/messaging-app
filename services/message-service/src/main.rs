use actix_web::{HttpServer, App};

use deadpool_postgres::tokio_postgres::NoTls;
use dotenv::dotenv;

use crate::config::AppConfig;

use ::config::Config;

mod routes;
mod handlers;
mod models;
mod db;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();

  let config_ = Config::builder()
    .add_source(::config::Environment::default())
    .build()
    .unwrap();

  let app_config: AppConfig = config_.try_deserialize().unwrap();

  let pool = app_config.pg.create_pool(None, NoTls).unwrap();

  HttpServer::new(move || {
    App::new()
      .configure(routes::init_routes)
  }).bind(app_config.server_address.to_owned()).expect(std::format!("Can not bind to {}", app_config.server_address).as_str())
    .run().await
}