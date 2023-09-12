use std::error::Error;
use log::info;
use env_logger::Env;

use actix_files as fs;
use actix_web::{ middleware::Logger, App, HttpServer };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

/// Starts serving from current working directory
pub async fn serve(bind_addr: String, tls_pair: Option<(String, String)>) -> Result<(), Box<dyn Error>> {
    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s"))
            .service(
                fs::Files::new("/static", "static")
                    .show_files_listing()
            )
            .service(
                fs::Files::new("/", "public")
                    .index_file("index.html")
            )
    });

    let http_server = if let Some((ref key, ref cert)) = tls_pair {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();

        http_server.bind_openssl(&bind_addr, builder)
    } else {
        http_server.bind(&bind_addr)
    }?;

    let http_server = http_server.run();

    info!("Listening on {}", &bind_addr);
    if let Some((key, cert)) = tls_pair { 
        info!("Using TLS key at {}", key);
        info!("Using TLS cert at {}", cert);
    }

    http_server
        .await?;

    Ok(())
}
