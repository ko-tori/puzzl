#[macro_use]
extern crate lazy_static;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_files as fs;
use std::sync::Mutex;
use serde::Deserialize;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html"]);
        tera
    };
}

async fn home() -> HttpResponse {
    HttpResponse::Ok()
    	.content_type("text/html")
    	.body(TEMPLATES.render("index.html", &Context::new()).expect("Failed to render template"))
}

struct AppState {

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let state = web::Data::new(AppState {
    });
    HttpServer::new(move || {
        App::new()
        	.app_data(state.clone())
            .route("/", web::get().to(home))
            .service(fs::Files::new("/static", "./static"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}