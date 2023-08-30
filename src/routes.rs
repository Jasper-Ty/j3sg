use actix_web::{ web, get, HttpResponse, http::header::ContentType};
use tera::Context;

use crate::state::AppState;

macro_rules! template_route {
    ($route: expr, $template: expr, $name: ident) => {
        #[get($route)]
        async fn $name(data: web::Data<AppState>) -> HttpResponse {
            let tera = data.tera.lock().unwrap();
            let mut context = Context::new();
            context.insert("dev_mode", &data.dev_mode);

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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(bio)
        .service(projects)
        .service(art)
        .service(notes)
        .service(blog)
        .service(misc);
}
