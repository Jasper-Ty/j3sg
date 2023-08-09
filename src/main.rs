use std::process::Command;

use actix_files as fs;
use actix_web::{ 
    post,
    App, 
    HttpServer,
    HttpResponse,
    Responder,
};
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };


/// Gets env variable or a default value if it doesn't exist
/// 
/// Returns a [`String`]
macro_rules! env_or {
    ($key:expr, $default:expr) => {
        std::env::var($key).unwrap_or(String::from($default))
    };
}


/// Runs a script that is supposed to pull changes, 
/// rebuild and restart the server.
///
/// I have a GitHub Webhook that hits this.
///
/// TODO: Secure this using GitHub's Webhook secret.
#[post("/update")]
async fn update() -> impl Responder {
    let update_script = std::env::var("JTY_WEBSITE_UPDATE_SCRIPT")
        .unwrap_or(String::from("ls"));
    let output = Command::new(update_script)
        .output()
        .ok() 
        .and_then(|o| String::from_utf8(o.stdout).ok());

    match output {
        Some(s) => HttpResponse::Ok().body(format!("Success\nOutput:\n{}", s, )),
        None => HttpResponse::InternalServerError().body("Unsuccessful."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = env_or!("JTY_WEBSITE_BIND_ADDR", "127.0.0.1:3000");
    let static_path = env_or!("JTY_WEBSITE_STATIC_PATH", "./static");
    let pages_path = env_or!("JTY_WEBSITE_PAGES_PATH", "./pages");
    let tls_key = env_or!("JTY_WEBSITE_TLS_KEY", "./dev/dev_key.pem");
    let tls_cert = env_or!("JTY_WEBSITE_TLS_CERT", "./dev/dev_cert.pem");

    println!("Listening on {}", bind_addr);
    println!("Static directory at {}", static_path);
    println!("Pages directory at {}", pages_path);
    println!("Using TLS key/cert {}/{}", tls_key, tls_cert);

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(tls_key, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(tls_cert).unwrap();

    HttpServer::new(move || {
        App::new()
            .service(update)
            .service(
                fs::Files::new("/static", &static_path[..])
                    .show_files_listing()
                    .index_file("index.html")
            )
            .service(
                fs::Files::new("/", &pages_path[..])
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = fs::NamedFile::open_async("./pages/404.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
            )
    })
    .bind_openssl(bind_addr, builder)?
    .run()
    .await
}
