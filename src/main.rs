use std::path::Path;

use actix_files as fs;
use actix_web::{ Error, get, App, HttpServer };
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use jty_website::env;

#[get("/index")]
async fn index() -> Result<fs::NamedFile, Error> {
    let path = Path::new(env!("OUT_DIR"))
        .join("pages")
        .join("index.html");
    let file = fs::NamedFile::open(path)?;
    Ok(file.use_last_modified(true))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Listening on {}", &env::BIND_ADDR[..]);
    println!("Static directory at {}", &env::STATIC_PATH[..]);
    println!("Pages directory at {}", &env::PAGES_PATH[..]);
    if let Some(env::TlsPair { ref key, ref cert }) = *env::TLS_PAIR { 
        println!("Using TLS key at {}", key);
        println!("Using TLS cert at {}", cert);
    }

    // TODO: move this into a factory
    let http_server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(
                fs::Files::new("/static", &env::STATIC_PATH[..])
                    .show_files_listing()
                    .index_file("index.html")
            )
            .service(
                fs::Files::new("/", &env::PAGES_PATH[..])
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = fs::NamedFile::open_async("./pages/404.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
            )
    });

    if let Some(env::TlsPair { ref key, ref cert }) = *env::TLS_PAIR {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();

        http_server.bind_openssl(&env::BIND_ADDR[..], builder)?
            .run()
            .await
    } else {
        http_server.bind(&env::BIND_ADDR[..])?
            .run()
            .await
    }
}
