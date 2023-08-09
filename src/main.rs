use actix_files as fs;
use actix_web::{ 
    post,
    App, 
    HttpServer,
    HttpResponse,
    Responder,
};
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };
use std::process::Command;

#[post("/update")]
async fn update() -> impl Responder {
    let update_script = std::env::var("JTY_WEBSITE_UPDATE_SCRIPT")
        .unwrap_or(String::from("ls"));
    Command::new(update_script)
        .output()
        .ok() 
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|o| HttpResponse::Ok().body(format!("Success. Output: {}", o)))
        .unwrap_or(HttpResponse::InternalServerError().body("Unsuccessful"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = std::env::var("JTY_WEBSITE_BIND_ADDR")
        .map(|s| (s, 80))
        .unwrap_or(("127.0.0.1".to_string(), 3000));
    println!("Listening on {}:{}", bind_addr.0, bind_addr.1);

    let static_path = std::env::var("JTY_WEBSITE_STATIC_PATH")
        .unwrap_or(String::from("./static"));
    let pages_path = std::env::var("JTY_WEBSITE_PAGES_PATH")
        .unwrap_or(String::from("./pages"));
    println!("Static directory at {}", static_path);
    println!("Pages directory at {}", pages_path);

    HttpServer::new(move || {
        App::new()
            .service(update)
            .service(
                fs::Files::new("/static", static_path.clone())
                    .show_files_listing()
                    .index_file("index.html")
            )
            .service(
                fs::Files::new("/", pages_path.clone())
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = fs::NamedFile::open_async("./pages/404.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
            )
    })
    .bind(bind_addr)?
    .run()
    .await
}
