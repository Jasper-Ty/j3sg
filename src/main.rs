use actix_files as fs;
use actix_web::{ web, App, HttpServer };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use jty_website::state::AppState;
use jty_website::config::Config;
use jty_website::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let Config {
        pages_path,
        static_path,
        bind_addr,
        tls_pair
    } = Config::get();

    std::env::set_var("RUST_LOG", "debug");

    println!("Listening on {}", bind_addr);
    println!("Static directory at {}", static_path);
    println!("Pages directory at {}", pages_path);
    if let Some((key, cert)) = tls_pair { 
        println!("Using TLS key at {}", key);
        println!("Using TLS cert at {}", cert);
    }

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new()))
            .service(routes::index)
            .service(routes::bio)
            .service(routes::projects)
            .service(routes::art)
            .service(routes::notes)
            .service(routes::blog)
            .service(routes::misc)
            .service(
                fs::Files::new("/static", static_path)
                    .show_files_listing()
                    .index_file("index.html")
            )
    });

    if let Some((key, cert)) = tls_pair {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();

        http_server.bind_openssl(bind_addr, builder)?
            .run()
            .await
    } else {
        http_server.bind(bind_addr)?
            .run()
            .await
    }
}
