pub mod routes;

/// Holds application state
pub mod state {
    use tera::Tera;
    use crate::config::Config;

    pub struct AppState {
        pub tera: Tera,
    }
    impl AppState {
        pub fn new() -> Self {
            let Config { pages_path, .. } = Config::get();
            let glob = format!("{}/**/*.html", pages_path);
            let mut tera = Tera::new(&glob).expect("Should be able to compile templates");
            tera.autoescape_on(vec![".html", ".sql"]);
            AppState {
                tera
            }
        }
    }
}

/// Holds various environment variables initialized behind a [`lazy_static`] macro. 
pub mod config {
    use lazy_static::lazy_static;

    macro_rules! env_or {
        ($key:expr, $default:expr) => {
            std::env::var($key).unwrap_or(String::from($default))
        };
    }
    lazy_static! {
        pub static ref STATIC_PATH: String = env_or!("JTY_WEBSITE_STATIC_PATH", "./static");
        pub static ref PAGES_PATH: String = env_or!("JTY_WEBSITE_PAGES_PATH", "./pages");
        pub static ref BIND_ADDR: String = env_or!("JTY_WEBSITE_BIND_ADDR", "127.0.0.1:5000");
        pub static ref TLS_KEY: String = env_or!("JTY_WEBSITE_TLS_KEY", "");
        pub static ref TLS_CERT: String = env_or!("JTY_WEBSITE_TLS_CERT", "");
        pub static ref TLS: bool = std::env::var("JTY_WEBSITE_TLS_KEY")
            .and(std::env::var("JTY_WEBSITE_TLS_CERT"))
            .is_ok();
    }
    pub struct Config<'a> {
        pub static_path: &'a str,
        pub pages_path: &'a str,
        pub bind_addr: &'a str,
        pub tls_pair: Option<(&'a str, &'a str)>,
    }
    impl<'a> Config<'a> {
        pub fn get() -> Self {
            Config {
                static_path: &STATIC_PATH[..],
                pages_path: &PAGES_PATH[..],
                bind_addr: &BIND_ADDR[..],
                tls_pair: (*TLS).then_some((&TLS_KEY[..], &TLS_CERT[..])),
            }
        }
    }
}
