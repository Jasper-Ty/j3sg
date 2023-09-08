use std::error::Error;
use log::{ info, debug };
use env_logger::Env;

use actix_files as fs;
use actix_web::{ middleware::Logger, App, HttpServer };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use j3sg_serve::config::Config;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let dev_mode = args.iter().any(|s| s == "-D" || s == "--dev");

    let Config {
        static_path,
        public_path,
        bind_addr,
        tls_pair,
    } = Config::get();

    env_logger::init_from_env(Env::default().default_filter_or("info"));


    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s"))
            .service(
                fs::Files::new("/static", static_path)
                    .show_files_listing()
            )
            .service(
                fs::Files::new("/", public_path)
                    .index_file("index.html")
            )
    });

    let http_server = if let Some((key, cert)) = tls_pair {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();

        http_server.bind_openssl(bind_addr, builder)
    } else {
        http_server.bind(bind_addr)
    }?;

    let http_server = http_server.run();

    info!("Listening on {}", bind_addr);
    info!("Static directory at {}", static_path);
    info!("Public directory at {}", public_path);
    if let Some((key, cert)) = tls_pair { 
        info!("Using TLS key at {}", key);
        info!("Using TLS cert at {}", cert);
    }
    if dev_mode {
        debug!("Running in dev mode.");
    } 

    http_server
        .await?;

    Ok(())
}
