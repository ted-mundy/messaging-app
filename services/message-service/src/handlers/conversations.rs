use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

// use crate::models::conversation;

#[derive(Deserialize)]
pub struct ConversationFilter {
  user_id: Option<usize>,
}

pub async fn get_conversations(_req: HttpRequest, info: web::Query<ConversationFilter>) -> HttpResponse {
  let user_id: Option<usize> = info.user_id;

  // let r = user_id.and_then(|id| {
  //   if id == 1 {
  //     Some("User 1's conversations")
  //   } else {
  //     None
  //   }
  // }).unwrap_or("No conversations found");

  HttpResponse::Ok().body("hello")
}
