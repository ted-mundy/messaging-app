use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use serde::Deserialize;

use crate::db::conversation_actions;

#[derive(Deserialize)]
pub struct ConversationFilter {
  user_id: Option<usize>,
}

pub async fn get_conversations(info: web::Query<ConversationFilter>, db_pool: web::Data<r2d2::Pool<ConnectionManager<diesel::PgConnection>>>) -> HttpResponse {
  let db_conn = db_pool.get().unwrap();

  let conversations = conversation_actions::get_conversations(db_conn).await;

  match conversations {
    Ok(conversations) => HttpResponse::Ok().json(conversations),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}
