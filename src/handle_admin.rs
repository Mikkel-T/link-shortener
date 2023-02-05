use crate::mongo::{delete_links, get_link, get_links, insert_link, update_link, Link};
use actix_identity::Identity;
use actix_web::{web, web::Redirect, HttpMessage, HttpRequest, HttpResponse};
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

pub async fn logout(id: Identity) -> Redirect {
    id.logout();
    Redirect::to("/")
}

pub async fn login(request: HttpRequest, pass: web::Form<Info>) -> Redirect {
    if pass.password == env::var("ADMIN_PASSWORD").unwrap() {
        Identity::login(&request.extensions(), "admin".into()).unwrap();
    };

    Redirect::to("/dash").see_other()
}

pub async fn auth_status(id: Option<Identity>) -> HttpResponse {
    if let Some(id) = id {
        HttpResponse::Ok().json(json!({"logged_in": true, "name": id.id().unwrap()}))
    } else {
        HttpResponse::Unauthorized().json(json!({"logged_in": false}))
    }
}

pub async fn add_link(
    _: Identity,
    links: web::Data<Collection<Link>>,
    link: web::Json<AddLinkBody>,
) -> HttpResponse {
    let mut slug = nanoid!(7);
    if let Some(s) = link.slug.clone() {
        if !s.is_empty() {
            slug = s
        }
    }

    match get_link(slug.clone(), &links).await {
                Some(_) => {
                    HttpResponse::Conflict()
                        .json(json!({"success": false, "message": format!("Link with slug \"{slug}\" already exists")}))
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

pub async fn fetch_links(_: Identity, links: web::Data<Collection<Link>>) -> HttpResponse {
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

    HttpResponse::Ok().json(json!(links))
}

pub async fn fetch_link(
    _: Identity,
    links: web::Data<Collection<Link>>,
    slug: web::Path<String>,
) -> HttpResponse {
    match get_link(slug.clone(), &links).await {
                Some(link) => {
                    HttpResponse::Ok().json(json!({"success": true, "link": ResponseLinkBody {
                        slug: link.slug.clone(),
                        url: link.url.clone(),
                        expires_uses: link.expires_uses,
                        expire_at: link.expire_at
                    }}))}
                None => {
                    HttpResponse::NotFound().json(json!({"success": false, "message": format!("Could not find link with the slug \"{slug}\"")}))
                }
            }
}

pub async fn delete(
    _: Identity,
    links: web::Data<Collection<Link>>,
    slug: web::Path<String>,
) -> HttpResponse {
    let deleted = delete_links(slug.into_inner(), &links).await.deleted_count;
    HttpResponse::Ok().json(json!({"success": true, "deleted": deleted}))
}

pub async fn update(
    _: Identity,
    links: web::Data<Collection<Link>>,
    body: web::Json<UpdateLinkBody>,
    slug: web::Path<String>,
) -> HttpResponse {
    if Url::parse(&body.url).is_err() {
        return HttpResponse::BadRequest().json(
            json!({"success": false, "message": format!("\"{}\" is not a valid url", body.url)}),
        );
    }

    let modified = update_link(
        slug.clone(),
        Link {
            slug: slug.into_inner(),
            url: body.url.clone(),
            expires_uses: body.expires_uses,
            expire_at: body.expire_at,
        },
        &links,
    )
    .await;
    HttpResponse::Ok().json(json!({"success": true, "modified": modified}))
}
