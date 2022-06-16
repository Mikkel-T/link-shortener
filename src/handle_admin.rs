use crate::mongo::{delete_links, get_link, get_links, insert_link, update_link, Link};
use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use mongodb::Collection;
use serde::Deserialize;
use serde_json::json;
use std::env;
use url::Url;

#[derive(Deserialize)]
pub struct Info {
    password: String,
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found()
        .insert_header((header::LOCATION, "/"))
        .finish()
}

pub async fn login(id: Identity, pass: web::Form<Info>) -> HttpResponse {
    if pass.password == env::var("ADMIN_PASSWORD").unwrap() {
        id.remember("admin".to_owned());
    };

    HttpResponse::Found()
        .insert_header((header::LOCATION, "/dash"))
        .finish()
}

pub async fn auth_status(id: Identity) -> HttpResponse {
    match id.identity() {
        Some(name) => HttpResponse::Ok().json(json!({"logged_in": true, "name": name})),
        None => HttpResponse::Unauthorized().json(json!({"logged_in": false})),
    }
}

pub async fn add_link(
    id: Identity,
    links: web::Data<Collection<Link>>,
    link: web::Json<Link>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            match get_link(link.slug.clone(), &links).await {
                Some(_) => {
                    HttpResponse::Conflict()
                        .json(json!({"success": false, "message": format!("Link with slug \"{}\" already exists", link.slug)}))
                },
                None => {
                    match Url::parse(&link.url).is_ok() {
                        true => {
                            let oid = insert_link(link.slug.clone(), link.url.clone(), &links, link.expires_uses).await;
                            HttpResponse::Created()
                                .json(json!({"success": true, "slug": link.slug, "url": link.url, "id": oid}))
                        }
                        false => {
                            HttpResponse::BadRequest().json(json!({"success": false, "message": format!("\"{}\" is not a valid url", link.url)}))
                        }
                    }
                },
            }
        }
        None => unauthorized(),
    }
}

pub async fn fetch_links(id: Identity, links: web::Data<Collection<Link>>) -> HttpResponse {
    match id.identity() {
        Some(_) => HttpResponse::Ok().json(json!(get_links(&links).await)),
        None => unauthorized(),
    }
}

pub async fn fetch_link(
    id: Identity,
    links: web::Data<Collection<Link>>,
    slug: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            match get_link(slug.clone(), &links).await {
                Some(link) => {
                    HttpResponse::Ok().json(json!({"success": true, "link": link}))}
                None => {
                    HttpResponse::NotFound().json(json!({"success": false, "message": format!("Coult not find link with the slug \"{slug}\"")}))
                }
            }
        }
        None => unauthorized(),
    }
}

pub async fn delete(
    id: Identity,
    links: web::Data<Collection<Link>>,
    slug: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let deleted = delete_links(slug.into_inner(), &links).await.deleted_count;
            HttpResponse::Ok().json(json!({"success": true, "deleted": deleted}))
        }
        None => unauthorized(),
    }
}

pub async fn update(
    id: Identity,
    links: web::Data<Collection<Link>>,
    body: web::Json<Link>,
    slug: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            if slug.clone() != body.slug {
                return HttpResponse::BadRequest().json(json!({"success": false, "message": format!("Slugs are not the same, the slug being edited ({slug}) is not the same as the updated \"{}\"", body.slug)}));
            }

            if Url::parse(&body.url).is_err() {
                return HttpResponse::BadRequest().json(json!({"success": false, "message": format!("\"{}\" is not a valid url", body.url)}));
            }

            let modified = update_link(slug.into_inner(), body.into_inner(), &links).await;
            HttpResponse::Ok().json(json!({"success": true, "modified": modified}))
        }
        None => unauthorized(),
    }
}

pub fn unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().body("You are not authorized to do this")
}
