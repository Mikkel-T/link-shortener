pub mod handle_admin;
pub mod mongo;

use actix_files::{Files, NamedFile};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{get, http::header, web, App, Either, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::future;
use mongo::{get_client, use_link, Link};
use mongodb::Collection;
use std::env;
use std::io::Error;
use std::path::Path;

#[get("/")]
async fn home() -> impl Responder {
    format!("Link shortener v{}", env!("CARGO_PKG_VERSION"))
}

#[get("/{slug}")]
async fn index(links: web::Data<Collection<Link>>, slug: web::Path<String>) -> HttpResponse {
    match use_link(slug.into_inner(), &links).await {
        Some(url) => HttpResponse::SeeOther()
            .insert_header((header::LOCATION, url))
            .finish(),
        None => HttpResponse::NotFound().body("404 Link not found"),
    }
}

#[get("/")]
async fn admin() -> Result<NamedFile, Error> {
    NamedFile::open("client/dist/index.html")
}

#[get("/login")]
async fn login(id: Identity) -> Either<HttpResponse, Result<NamedFile, Error>> {
    match id.identity() {
        Some(_) => Either::Left(
            HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/dash"))
                .finish(),
        ),
        None => Either::Right(NamedFile::open("client/dist/login/index.html")),
    }
}

#[get("/dash")]
async fn dash(id: Identity) -> Either<HttpResponse, Result<NamedFile, Error>> {
    match id.identity() {
        Some(_) => Either::Right(NamedFile::open("client/dist/dash/index.html")),
        None => Either::Left(
            HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/"))
                .finish(),
        ),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let paths = vec![
        "client/dist/login/index.html",
        "client/dist/dash/index.html",
        "client/dist/index.html",
    ];
    let mut missing_paths = Vec::new();
    for path in paths {
        if !Path::new(path).exists() {
            missing_paths.push(path)
        }
    }

    if missing_paths.len() > 0 {
        println!("Missing the following HTML files. Aborting");
        for path in missing_paths {
            println!("{path}");
        }
        std::process::exit(1);
    }

    let env_vars = vec!["PASSWORD_KEY", "MONGO_CONNECTION_STRING", "ADMIN_PASSWORD"];
    let mut missing_env_vars = Vec::new();

    for var in env_vars {
        if !env::var(var).is_ok() {
            missing_env_vars.push(var);
        }
    }

    if missing_env_vars.len() > 0 {
        println!("Missing the following environment variables. Aborting");
        for var in missing_env_vars {
            println!("{var}");
        }
        std::process::exit(1);
    }

    let client = get_client().await;
    let db = client.database("link-shortener");
    let links = db.collection::<Link>("links");

    let s1 = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(links.clone()))
            .service(home)
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run();

    let links = db.collection::<Link>("links");
    let s2 = HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(env::var("PASSWORD_KEY").unwrap().as_bytes())
                    .name("auth")
                    .secure(false),
            ))
            .app_data(web::Data::new(links.clone()))
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
                            .route(web::get().to(handle_admin::fetch_link))
                            .route(web::patch().to(handle_admin::update))
                            .route(web::delete().to(handle_admin::delete)),
                    )
                    .service(web::resource("/login").route(web::post().to(handle_admin::login)))
                    .service(web::resource("/logout").route(web::get().to(handle_admin::logout)))
                    .service(
                        web::resource("/auth/status")
                            .route(web::get().to(handle_admin::auth_status)),
                    ),
            )
            .service(Files::new("/", "client/dist"))
    })
    .bind("0.0.0.0:8081")?
    .run();

    future::try_join(s1, s2).await?;

    Ok(())
}
