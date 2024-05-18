pub mod messages;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/api")
    .configure(messages::init_routes));
}
