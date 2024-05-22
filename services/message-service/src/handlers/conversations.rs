use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};

use crate::db::conversation_actions::{self, ConversationFilter};

pub async fn get_conversations(info: web::Query<ConversationFilter>, db_pool: web::Data<r2d2::Pool<ConnectionManager<diesel::PgConnection>>>) -> HttpResponse {
  let db_conn = db_pool.get().unwrap();

  let conversations = conversation_actions::get_conversations(conversation_actions::GetConversationConfig {
    db_pool: db_conn,
    filter: info.into_inner(),
    page: 1,
    per_page: 10,
  }).await;

  HttpResponse::Ok().json(conversations)
}
