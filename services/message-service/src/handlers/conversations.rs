use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Conversation {
  id: usize,
  // user_id: usize,
  // messages: Vec<String>,
}

pub async fn get_conversations(_req: HttpRequest, info: web::Query<Conversation>) -> HttpResponse {
  // let user_id = _req.match_info().get("user_id").unwrap_or_else(|| "");
  // println!("User ID: {}", user_id);
  // if user_id == "" {
  //   return HttpResponse::BadRequest().body("User ID is required");
  // }
  // if user_id.parse::<usize>().is_err() {
  //   return HttpResponse::BadRequest().body("User ID must be a number");
  // }

  // HttpResponse::Ok().body(format!("GET conversations for user {}", user_id))

  // HttpResponse::Ok().json(json.0)

  HttpResponse::Ok().json(info.id)
}
