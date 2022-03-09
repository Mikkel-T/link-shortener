pub mod handle_admin;
pub mod mongo;

use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, http::header, web, App, Either, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongo::{get_client, get_link, Link};
use mongodb::Collection;
use std::env;
use std::io::Error;

#[get("/")]
async fn home() -> impl Responder {
    "Link shortener"
}

#[get("/{slug}")]
async fn index(links: web::Data<Collection<Link>>, slug: web::Path<String>) -> HttpResponse {
    match get_link(slug.into_inner(), &links).await {
        Some(url) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, url))
            .finish(),
        None => HttpResponse::NotFound().body("404 Link not found"),
    }
}

#[get("/admin")]
async fn admin(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(_) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/admin/dash"))
            .finish(),
        None => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/admin/login"))
            .finish(),
    }
}

#[get("/admin/login")]
async fn login(id: Identity) -> Either<HttpResponse, Result<NamedFile, Error>> {
    match id.identity() {
        Some(_) => Either::Left(
            HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/admin"))
                .finish(),
        ),
        None => Either::Right(NamedFile::open("client/dist/admin/login/index.html")),
    }
}

#[get("/admin/dash")]
async fn dash(id: Identity) -> Either<HttpResponse, Result<NamedFile, Error>> {
    match id.identity() {
        Some(_) => Either::Right(NamedFile::open("client/dist/admin/dash/index.html")),
        None => Either::Left(
            HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/admin"))
                .finish(),
        ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let client = get_client().await;
    let db = client.database("link-shortener");
    let links = db.collection::<Link>("links");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(env::var("PASSWORD_KEY").unwrap().as_bytes())
                    .name("auth")
                    .secure(false),
            ))
            .app_data(web::Data::new(links.clone()))
            .service(home)
            .service(admin)
            .service(login)
            .service(dash)
            .service(
                web::scope("/api/admin")
                    .service(
                        web::resource("/links")
                            .route(web::get().to(handle_admin::fetch_links))
                            .route(web::post().to(handle_admin::add_link)),
                    )
                    .service(
                        web::resource("/links/{slug}")
                            .route(web::patch().to(handle_admin::update))
                            .route(web::delete().to(handle_admin::delete)),
                    )
                    .service(web::resource("/login").route(web::post().to(handle_admin::login)))
                    .service(web::resource("/logout").route(web::get().to(handle_admin::logout))),
            )
            .service(Files::new("/assets", "client/dist/assets"))
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
