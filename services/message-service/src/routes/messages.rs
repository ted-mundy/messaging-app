use actix_web::web;

use crate::handlers::messages::get_messages;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/messages")
      .route("", web::get().to(get_messages))
  );
}
