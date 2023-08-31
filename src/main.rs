use std::error::Error;
use log::{ info, debug, error };
use env_logger::Env;

use actix_files as fs;
use actix_web::{ middleware::Logger, web, App, HttpServer };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };
use notify::{ Watcher, RecursiveMode };

use jty_website::state::{ AppState, ADDRS };
use jty_website::config::Config;
use jty_website::routes;
use jty_website::reload::{ reload, ReloadMessage };

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let dev_mode = args.iter().any(|s| s == "-D" || s == "--dev");

    let Config {
        pages_path,
        static_path,
        bind_addr,
        tls_pair
    } = Config::get();

    env_logger::init_from_env(Env::default().default_filter_or("info"));


    let data = web::Data::new(AppState::new(dev_mode));

    let watcher_data = data.clone();
    let mut watcher = notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
        let event = res.unwrap();    
        if event.kind.is_modify() {
            info!(target:"watcher", "Change detected. Reloading templates...");
            let mut tera = watcher_data.tera.lock().unwrap();
 
            match (*tera).full_reload() {
                Ok(_) => {
                    info!(target:"watcher", "Successfully reloaded");
                    let addrs = ADDRS.lock().unwrap();
                    for addr in addrs.iter() {
                        addr.do_send(ReloadMessage);
                    }
                },
                Err(e) => println!("{}", e.to_string()),
            };
        }
    })?;

    watcher.watch(std::path::Path::new(pages_path), RecursiveMode::Recursive)?;

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a \"%r\" %s"))
            .app_data(data.clone())
            .configure(routes::config)
            .service(reload)
            .service(
                fs::Files::new("/static", static_path)
                    .show_files_listing()
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
    info!("Pages directory at {}", pages_path);
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
