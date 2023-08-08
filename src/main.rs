use actix_files as fs;
use actix_web::{ App, HttpServer };
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = std::env::var("JTY_WEBSITE_HOST")
        .map(|s| (s, 80))
        .unwrap_or(("127.0.0.1".to_string(), 3000));
    println!("Listening on {}:{}", host.0, host.1);

    let static_path = std::env::var("JTY_WEBSITE_STATIC_PATH")
        .unwrap_or(String::from("./static"));
    let pages_path = std::env::var("JTY_WEBSITE_PAGES_PATH")
        .unwrap_or(String::from("./pages"));
    println!("Static directory at {}", static_path);
    println!("Pages directory at {}", pages_path);

    HttpServer::new(move || {
        App::new()
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
    .bind(host)?
    .run()
    .await
}
