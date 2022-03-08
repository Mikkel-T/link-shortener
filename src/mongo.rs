use futures::TryStreamExt;
use mongodb::bson::{doc, from_bson, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub slug: String,
    pub url: String,
}

pub async fn get_client() -> Client {
    let client_options = ClientOptions::parse(env::var("MONGO_CONNECTION_STRING").unwrap())
        .await
        .unwrap();

    Client::with_options(client_options).unwrap()
}

pub async fn insert_link(slug: String, link: String, collection: &Collection<Link>) -> String {
    from_bson::<ObjectId>(
        collection
            .insert_one(
                Link {
                    slug: slug,
                    url: link,
                },
                None,
            )
            .await
            .unwrap()
            .inserted_id,
    )
    .unwrap()
    .to_hex()
}

pub async fn get_link(slug: String, collection: &Collection<Link>) -> Option<String> {
    match collection
        .find_one(doc! {"slug": slug}, None)
        .await
        .unwrap()
    {
        Some(s) => Some(s.url),
        None => None,
    }
}

pub async fn get_links(collection: &Collection<Link>) -> Vec<Link> {
    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => return vec![],
    };

    cursor.try_collect().await.unwrap_or_else(|_| vec![])
}

pub async fn delete_links(
    slug: String,
    collection: &Collection<Link>,
) -> mongodb::results::DeleteResult {
    collection
        .delete_many(doc! {"slug": slug}, None)
        .await
        .unwrap()
}

pub async fn update_link(
    slug: String,
    new_url: String,
    collection: &Collection<Link>,
) -> mongodb::results::UpdateResult {
    collection
        .update_one(doc! {"slug": &slug}, doc! {"$set": {"url": new_url}}, None)
        .await
        .unwrap()
}