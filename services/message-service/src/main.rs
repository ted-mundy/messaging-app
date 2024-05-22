use actix_web::{HttpServer, App, web};

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
    .set_default("max_connections", 5).unwrap()
    .build()
    .unwrap();

  let app_config: AppConfig = config_.try_deserialize().unwrap();

  let pool = db::create_pool(app_config.to_owned());

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .configure(routes::init_routes)
    })
      .bind(app_config.server_address.to_owned()).expect(std::format!("Can not bind to {}", app_config.server_address).as_str())
      .run().await
}
