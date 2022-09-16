use actix_web::web::{Data};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Client, Collection, Database, IndexModel,
};
use std::env;

use crate::{
    model::{blog::Blog, user::User},
    utils::hash_password,
};

pub fn get_db(client: Data<Client>) -> Database {
    let database_name: String = env::var("MONGO_DB_NAME").unwrap();
    client.database(&database_name)
}

pub fn get_user_collection(client: Data<Client>) -> Collection<User> {
    get_db(client).collection("users")
}

pub fn get_blog_collection(client: Data<Client>) -> Collection<Blog> {
    get_db(client).collection("blogs")
}

pub async fn create_blog_index(client: &Client) {
    let model = IndexModel::builder().keys(doc! { "title": 1 }).build();
    let database_name: String = env::var("MONGO_DB_NAME").unwrap();
    client
        .database(&database_name)
        .collection::<Blog>("blogs")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

pub async fn create_init_user(client: &Client) {
    let database_name: String = env::var("MONGO_DB_NAME").unwrap();
    let user_col = client.database(&database_name).collection::<User>("users");
    let counts = user_col.estimated_document_count(None).await.unwrap();
    if counts == 0 {
        let hash_password = hash_password(&env::var("INIT_PASSWORD").unwrap());
        let new_user_obj = User {
            password: hash_password,
            created_at: Some(DateTime::now()),
            _id: Some(ObjectId::new()),
            email: env::var("INIT_EMAIL").unwrap(),
            username: env::var("INIT_USERNAME").unwrap(),
        };
        user_col.insert_one(new_user_obj, None).await.expect("should create init user!");
    }
}
