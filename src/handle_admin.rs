use crate::mongo::{delete_links, get_link, get_links, insert_link, update_link, Link};
use actix_identity::Identity;
use actix_web::{http::header, web, HttpResponse};
use mongodb::Collection;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use url::Url;

#[derive(Deserialize)]
pub struct Info {
    password: String,
}

#[derive(Deserialize)]
pub struct AddLinkBody {
    slug: Option<String>,
    url: String,
    expires_uses: Option<usize>,
    expire_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateLinkBody {
    url: String,
    expires_uses: Option<usize>,
    expire_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct ResponseLinkBody {
    slug: String,
    url: String,
    expires_uses: Option<usize>,
    expire_at: Option<chrono::DateTime<chrono::Utc>>,
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
    link: web::Json<AddLinkBody>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            let mut slug = nanoid!(7);
            if let Some(s) = link.slug.clone() {
                if !s.is_empty() {
                    slug = s
                }
            }

            match get_link(slug.clone(), &links).await {
                Some(_) => {
                    HttpResponse::Conflict()
                        .json(json!({"success": false, "message": format!("Link with slug \"{}\" already exists", slug)}))
                },
                None => {
                    if Url::parse(&link.url).is_err() {
                        return HttpResponse::BadRequest().json(json!({"success": false, "message": format!("\"{}\" is not a valid url", link.url)}));
                    }

                    let oid = insert_link(slug.clone(), link.url.clone(), &links, link.expires_uses, link.expire_at).await;
                    HttpResponse::Created().json(json!({"success": true, "slug": slug, "url": link.url, "id": oid}))
                },
            }
        }
        None => unauthorized(),
    }
}

pub async fn fetch_links(id: Identity, links: web::Data<Collection<Link>>) -> HttpResponse {
    let links: Vec<ResponseLinkBody> = get_links(&links)
        .await
        .iter()
        .map(|link| ResponseLinkBody {
            slug: link.slug.clone(),
            url: link.url.clone(),
            expires_uses: link.expires_uses,
            expire_at: link.expire_at,
        })
        .collect();
    match id.identity() {
        Some(_) => HttpResponse::Ok().json(json!(links)),
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
                    HttpResponse::Ok().json(json!({"success": true, "link": ResponseLinkBody {
                        slug: link.slug.clone(),
                        url: link.url.clone(),
                        expires_uses: link.expires_uses,
                        expire_at: link.expire_at
                    }}))}
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
    body: web::Json<UpdateLinkBody>,
    slug: web::Path<String>,
) -> HttpResponse {
    match id.identity() {
        Some(_) => {
            if Url::parse(&body.url).is_err() {
                return HttpResponse::BadRequest().json(json!({"success": false, "message": format!("\"{}\" is not a valid url", body.url)}));
            }

            let modified = update_link(
                slug.clone(),
                Link {
                    slug: slug.into_inner(),
                    url: body.url.clone(),
                    expires_uses: body.expires_uses.clone(),
                    expire_at: body.expire_at.clone(),
                },
                &links,
            )
            .await;
            HttpResponse::Ok().json(json!({"success": true, "modified": modified}))
        }
        None => unauthorized(),
    }
}

pub fn unauthorized() -> HttpResponse {
    HttpResponse::Unauthorized().body("You are not authorized to do this")
}
