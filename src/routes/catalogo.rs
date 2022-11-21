use actix_web::*;

pub async fn catalogo() -> HttpResponse {    
    HttpResponse::Ok()
    .content_type("application/json")
    .body(r#"[
        {"msg":"olá aj", "autor": "ajgf@gmail.com"}
        ,{"msg":"olá ana", "autor": "ana@gmail.com"}
        ]"#)
}