use actix_web::{HttpRequest, HttpResponse};

pub async fn get_messages(_req: HttpRequest) -> HttpResponse {
  // let user_id = _req.match_info().get("user_id")
  HttpResponse::Ok().body("GET messages")
}
