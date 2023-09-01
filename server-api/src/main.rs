use actix_cors::Cors;
use actix_web::{HttpServer, App, web, HttpResponse, Responder, get, post};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("SERVIDOR ARRANCADO");
    
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .wrap(cors)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}