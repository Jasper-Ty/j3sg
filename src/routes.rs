use actix_web::{ web, get, HttpResponse, http::header::ContentType };
use tera::Context;

use crate::{config::Config, state::AppState};

macro_rules! template_route {
    ($route: expr, $template: expr, $name: ident) => {
        #[get($route)]
        async fn $name(data: web::Data<AppState>) -> HttpResponse {
            let tera = &data.tera;
            let context = Context::new();
            let rendered_html = tera.render($template, &context)
                .unwrap_or_else(|e| e.to_string());
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(rendered_html)
        }
    }
}

template_route!("/", "index.html", index);
template_route!("/bio", "bio.html", bio);
template_route!("/projects", "projects.html", projects);
template_route!("/art", "art.html", art);
template_route!("/notes", "notes.html", notes);
template_route!("/blog", "blog.html", blog);
template_route!("/misc", "misc.html", misc);

#[get("/config")]
async fn config() -> String {
    let Config { pages_path, static_path, bind_addr, .. } = Config::get();
    format!("{} {} {}", pages_path, static_path, bind_addr)
}