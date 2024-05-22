use actix_web::{web, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};

use log::error;

use crate::db::conversation_actions::{self, ConversationFilter, CreateConversationInput};


pub async fn get_conversations(info: web::Query<ConversationFilter>, db_pool: web::Data<r2d2::Pool<ConnectionManager<diesel::PgConnection>>>) -> HttpResponse {
  let db_conn = db_pool.get();
  match db_conn {
    Ok(conn) => {
      let conversations = conversation_actions::get_conversations_from_db(conversation_actions::GetConversationConfig {
        db_pool: conn,
        filter: info.into_inner(),
        page: 1,
        per_page: 10,
      }).await;

      HttpResponse::Ok().json(conversations)
    },
    Err(e) => {
      error!("Failed to get connection from pool: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}

pub async fn create_conversation(info: web::Json<CreateConversationInput>, db_pool: web::Data<r2d2::Pool<ConnectionManager<diesel::PgConnection>>>) -> HttpResponse {
  let db_conn = db_pool.get();
  match db_conn {
    Ok(conn) => {
      let conversation = conversation_actions::create_conversation(conversation_actions::CreateConversationConfig {
        db_pool: conn,
        input: info.into_inner(),
      }).await;

      match conversation {
        Ok(conversation) => HttpResponse::Ok().json(conversation),
        Err(e) => {
          error!("Failed to create conversation: {}", e);
          HttpResponse::InternalServerError().finish()
        }
      }
    },
    Err(e) => {
      error!("Failed to get connection from pool: {}", e);
      HttpResponse::InternalServerError().finish()
    }
  }
}
