use actix_web::*;

mod routes;
use routes::ping::*;
use routes::index::*;
use routes::catalogo::*;
use routes::info::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(||{
        App::new()
        .route("/", web::get().to(index))
        .route("/info", web::get().to(info))
        .route("/catalogo", web::get().to(catalogo))
        .route("/ping", web::get().to(ping))
    });
    let port = 8081;
    let api = api.bind(format!("127.0.0.1:{}",port))
    .expect("não foi possivel conecta!!");

    println!("conectado com sucesso! em http://127.0.0.1:{}",port);

    api.run().await
}
