use actix_web::HttpResponse;

pub async fn get_messages() -> HttpResponse {
  HttpResponse::Ok().body("GET messages")
}
