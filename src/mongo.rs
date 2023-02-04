use futures::TryStreamExt;
use mongodb::bson::{doc, from_bson, oid::ObjectId};
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub slug: String,
    pub url: String,
    pub expires_uses: Option<usize>,
    #[serde_as(as = "Option<mongodb::bson::DateTime>")]
    pub expire_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_client() -> Client {
    let client_options = ClientOptions::parse(env::var("MONGO_CONNECTION_STRING").unwrap())
        .await
        .unwrap();

    Client::with_options(client_options).unwrap()
}

pub async fn insert_link(
    slug: String,
    link: String,
    collection: &Collection<Link>,
    expires_uses: Option<usize>,
    expire_at: Option<chrono::DateTime<chrono::Utc>>,
) -> String {
    from_bson::<ObjectId>(
        collection
            .insert_one(
                Link {
                    slug,
                    url: link,
                    expires_uses,
                    expire_at,
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

pub async fn use_link(slug: String, collection: &Collection<Link>) -> Option<String> {
    let link = collection
        .find_one(doc! {"slug": &slug}, None)
        .await
        .unwrap();
    match link {
        Some(l) => {
            if let Some(uses) = l.expires_uses {
                if uses > 1 {
                    collection
                        .update_one(
                            doc! {"slug": &slug},
                            doc! {"$inc": {"expires_uses": -1}},
                            None,
                        )
                        .await
                        .unwrap();
                } else {
                    delete_links(slug, collection).await;
                }
            }
            Some(l.url)
        }
        None => None,
    }
}

pub async fn get_link(slug: String, collection: &Collection<Link>) -> Option<Link> {
    collection
        .find_one(doc! {"slug": &slug}, None)
        .await
        .unwrap()
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
    new_link: Link,
    collection: &Collection<Link>,
) -> mongodb::results::UpdateResult {
    collection
        .replace_one(doc! {"slug": &slug}, new_link, None)
        .await
        .unwrap()
}
