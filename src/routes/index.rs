use actix_web::*;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("index da api")
}