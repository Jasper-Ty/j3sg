use actix_web::{ 
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("jasper")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("JTY_WEBSITE_HOST")
        .map(|host| (host, 80))
        .unwrap_or(("127.0.0.1".to_string(), 8080));

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(host)?
    .run()
    .await
}
