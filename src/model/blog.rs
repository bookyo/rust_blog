use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use validator::Validate;


#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Blog {
  pub _id: Option<ObjectId>,
  #[validate(length(min = 1))]
  pub title: String,
  #[validate(length(min = 5))]
  pub content: String,
  pub created_at: Option<DateTime>
}
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BlogOneQuery {
  pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BlogQuery {
  pub q: Option<String>,
  pub limit: u16,
  pub page: u16
}