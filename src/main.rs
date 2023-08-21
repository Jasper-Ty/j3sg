use actix_files as fs;
use actix_web::{ Error, get, App, HttpServer };
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };

use lazy_static::lazy_static;

#[derive(Clone)]
pub struct TlsPair {
    pub key: String,
    pub cert: String,
}
macro_rules! env_or {
    ($key:expr, $default:expr) => {
        std::env::var($key).unwrap_or(String::from($default))
    };
}
lazy_static! {
    pub static ref STATIC_PATH: String = env_or!("JTY_WEBSITE_STATIC_PATH", "./static");
    pub static ref PAGES_PATH: String = env_or!("JTY_WEBSITE_PAGES_PATH", "./pages");
    pub static ref BIND_ADDR: String = env_or!("JTY_WEBSITE_BIND_ADDR", "127.0.0.1:5000");
    pub static ref TLS_PAIR: Option<TlsPair> = std::env::var("JTY_WEBSITE_TLS_KEY")
    .and_then(|key| std::env::var("JTY_WEBSITE_TLS_CERT")
              .map(|cert| TlsPair { key, cert }))
    .ok();
}

#[get("/index")]
async fn index() -> Result<fs::NamedFile, Error> {
    let file = fs::NamedFile::open("./pages/index.html")?;
    Ok(file.use_last_modified(true))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Listening on {}", &BIND_ADDR[..]);
    println!("Static directory at {}", &STATIC_PATH[..]);
    println!("Pages directory at {}", &PAGES_PATH[..]);
    if let Some(TlsPair { ref key, ref cert }) = *TLS_PAIR { 
        println!("Using TLS key at {}", key);
        println!("Using TLS cert at {}", cert);
    }


    // TODO: move this into a factory
    let http_server = HttpServer::new(move || {
        App::new()
            .service(index)
            .service(
                fs::Files::new("/static", &STATIC_PATH[..])
                    .show_files_listing()
                    .index_file("index.html")
            )
            .service(
                fs::Files::new("/", &PAGES_PATH[..])
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = fs::NamedFile::open_async("./pages/404.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    }))
            )
    });

    if let Some(TlsPair { ref key, ref cert }) = *TLS_PAIR {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(cert).unwrap();

        http_server.bind_openssl(&BIND_ADDR[..], builder)?
            .run()
            .await
    } else {
        http_server.bind(&BIND_ADDR[..])?
            .run()
            .await
    }
}
