use actix_web::web;

use crate::handlers::conversations::get_conversations;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/conversations")
      .route("", web::get().to(get_conversations))
  );
}
