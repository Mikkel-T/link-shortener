use crate::mongo::{delete_links, get_links, insert_link, update_link, Link};
use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use mongodb::Collection;
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize)]
pub struct Info {
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateBody {
    new_url: String,
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/admin"))
        .finish()
}

pub async fn login(id: Identity, pass: web::Form<Info>) -> HttpResponse {
    if pass.password == env::var("ADMIN_PASSWORD").unwrap() {
        id.remember("admin".to_owned());
    };

    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/admin"))
        .finish()
}

pub async fn add_link(
    id: Identity,
    links: web::Data<Collection<Link>>,
    link: web::Json<Link>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let oid = insert_link(link.slug.clone(), link.url.clone(), &links).await;
            HttpResponse::Created()
                .json(json!({"success": true, "slug": link.slug, "url": link.url, "id": oid}))
        }
        None => HttpResponse::Unauthorized().body("You are not authorized to do this"),
    }
}

pub async fn fetch_links(id: Identity, links: web::Data<Collection<Link>>) -> HttpResponse {
    match id.identity() {
        Some(_) => HttpResponse::Ok().json(json!(get_links(&links).await)),
        None => HttpResponse::Unauthorized().body("You are not authorized to do this"),
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
        None => HttpResponse::Unauthorized().body("You are not authorized to do this"),
    }
}

pub async fn update(
    id: Identity,
    links: web::Data<Collection<Link>>,
    body: web::Json<UpdateBody>,
    slug: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let modified = update_link(slug.into_inner(), body.new_url.clone(), &links).await;
            HttpResponse::Ok().json(json!({"success": true, "modified": modified}))
        }
        None => HttpResponse::Unauthorized().body("You are not authorized to do this"),
    }
}
