use std::{env, fs, path::Path};

use tera::{ Tera, Context };

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pages_path = Path::new(&out_dir).join("pages");
    if !pages_path.exists() {
        fs::create_dir(&pages_path).unwrap();
    } 

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();
    context.insert("page_title", "Jasper Ty");
    context.insert("header_content", "Jasper Ty");
    context.insert("main_content", "Jasper Ty");

    let index_text = tera.render("index.html", &context).unwrap();
    
    let out_path = pages_path.join("index.html");
    if !out_path.exists() {
        fs::write(out_path, index_text).unwrap();
    } 
}
