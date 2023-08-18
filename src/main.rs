use actix_files as fs;
use actix_web::{ 
    App, 
    HttpServer,
};
use actix_web::dev::{ ServiceRequest, ServiceResponse, fn_service };
use openssl::ssl::{ SslAcceptor, SslFiletype, SslMethod };
use tera::{ Tera, Context };

/// Gets env variable or a default value if it doesn't exist
/// 
/// Returns a [`String`]
macro_rules! env_or {
    ($key:expr, $default:expr) => {
        std::env::var($key).unwrap_or(String::from($default))
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_addr = env_or!("JTY_WEBSITE_BIND_ADDR", "127.0.0.1:5000");
    let static_path = env_or!("JTY_WEBSITE_STATIC_PATH", "./static");
    let pages_path = env_or!("JTY_WEBSITE_PAGES_PATH", "./pages");

    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("page_title", "Jasper Ty");
    context.insert("header_content", "Header");
    context.insert("main_content", "Main");
    let index = tera.render("index.html", &context).unwrap();
    println!("{}", index);


    println!("Listening on {}", bind_addr);
    println!("Static directory at {}", static_path);
    println!("Pages directory at {}", pages_path);


    let http_server = HttpServer::new(move || {
        App::new()
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
    });


    if let Ok(tls_key) = std::env::var("JTY_WEBSITE_TLS_KEY") {
        let tls_cert = std::env::var("JTY_WEBSITE_TLS_CERT").unwrap();
        println!("Using TLS key/cert {}/{}", tls_key, tls_cert);
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(tls_key, SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(tls_cert).unwrap();

        http_server.bind_openssl(bind_addr, builder)?
            .run()
            .await
    } else {
        http_server.bind(bind_addr)?
            .run()
            .await
    }
}
