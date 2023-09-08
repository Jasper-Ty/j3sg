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
        pub static ref PUBLIC_PATH: String = env_or!("JTY_WEBSITE_PAGES_PATH", "./public");
        pub static ref BIND_ADDR: String = env_or!("JTY_WEBSITE_BIND_ADDR", "127.0.0.1:5000");
        pub static ref TLS_KEY: String = env_or!("JTY_WEBSITE_TLS_KEY", "");
        pub static ref TLS_CERT: String = env_or!("JTY_WEBSITE_TLS_CERT", "");
        pub static ref TLS: bool = std::env::var("JTY_WEBSITE_TLS_KEY")
            .and(std::env::var("JTY_WEBSITE_TLS_CERT"))
            .is_ok();
    }
    pub struct Config<'a> {
        pub static_path: &'a str,
        pub public_path: &'a str,
        pub bind_addr: &'a str,
        pub tls_pair: Option<(&'a str, &'a str)>,
    }
    impl<'a> Config<'a> {
        pub fn get() -> Self {
            Config {
                static_path: &STATIC_PATH[..],
                public_path: &PUBLIC_PATH[..],
                bind_addr: &BIND_ADDR[..],
                tls_pair: (*TLS).then_some((&TLS_KEY[..], &TLS_CERT[..])),
            }
        }
    }
}
