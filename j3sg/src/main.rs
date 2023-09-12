use env_logger::Env;
use std::error::Error;
use std::path::Path;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let src_dir = Path::new("src");
    let _static_dir = Path::new("static");
    let out_dir = Path::new("public");
    let template_dir = Path::new("templates");

    let verb = match args.get(1) {
        Some(s) => { match &s[..] {
            "gen" | "generate" | "G" => Verb::Generate,
            "serve" | "S" => Verb::Serve {
                bind: "127.0.0.1:5000".to_string(),
                tls: None
            },
            _ => Verb::Help 
        } },
        None => Verb::Help,
    };

    match verb {
        Verb::Generate => {
            j3sg_gen::generate(src_dir, out_dir, template_dir)?;
        }
        Verb::Serve { bind, tls } => {
            j3sg_serve::serve(bind, tls).await?;
        }
        Verb::Help => {
            println!("USAGE: j3sg COMMAND");
            println!("");
            println!("COMMANDS:");
            println!("    gen | generate | G");
            println!("        Compiles the static site into ./public");
            println!("    srv | serve | S");
            println!("        Serves files from ./public and ./static");
        }
    }

    Ok(())
}

enum Verb {
    Generate,
    Serve {
        bind: String,
        tls: Option<(String, String)>,
    },
    Help,
}
