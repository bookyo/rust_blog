use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResponseMessage {
  pub success: u8,
  pub message: String,
}
