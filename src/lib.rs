pub mod env {
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
}
