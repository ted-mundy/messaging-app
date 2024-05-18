use actix_web::{HttpServer, App};

mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    App::new()
      .configure(routes::init_routes)
  }).bind("0.0.0.0:6065").expect("Failed to bind to port 6065")
    .run().await
}