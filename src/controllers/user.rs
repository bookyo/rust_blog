use actix_web::{web, HttpResponse};
use log::info;
use mongodb::{Client, bson::{DateTime, doc, oid::ObjectId}};
use actix_web_validator::Json;
use serde::{Deserialize, Serialize};

use crate::{model::{user::{User, LoginUser, Claims, AuthorizedUser}, helper::ResponseMessage}, database::get_user_collection, utils::{hash_password, verify_password}};

pub async fn hello(_client: web::Data<Client>, authorized_user: Option<AuthorizedUser>,) -> HttpResponse {
  info!{"{:?}", authorized_user};
  HttpResponse::Ok().body(format!("Hello world!"))
}

#[derive(Deserialize, Serialize)]
struct LoginMessage {
  success: u8,
  token: String,
}

pub async fn register(client: web::Data<Client>, user: Json<User>) -> HttpResponse {
  let user_collection = get_user_collection(client);
  let user_obj = user.into_inner();
  let taken_user = user_collection.find_one(doc! {
    "username": &user_obj.username
  }, None).await.unwrap();
  if !taken_user.is_none() {
    return HttpResponse::BadRequest().json(ResponseMessage {
      success: 0,
      message: String::from("对不起，用户名已经存在！")
    })
  }
  let taken_user = user_collection.find_one(doc! {
    "email": &user_obj.email
  }, None).await.unwrap();
  if !taken_user.is_none() {
    return HttpResponse::Ok().json(ResponseMessage {
      success: 0,
      message: String::from("对不起，邮箱名已经存在！")
    })
  }
  let hash_password = hash_password(&user_obj.password);
  let new_user_obj = User {
    password: hash_password,
    created_at: Some(DateTime::now()),
    _id: Some(ObjectId::new()),
    ..user_obj
  };
  let user = user_collection.insert_one(new_user_obj, None).await;
  match user {
    Ok(insert) => HttpResponse::Ok().json(insert),
    Err(message) => HttpResponse::BadRequest().json(ResponseMessage {success: 0, message: message.to_string()}),
  }
}

pub async fn login(client: web::Data<Client>, user: Json<LoginUser>) -> HttpResponse {
  let user_collection = get_user_collection(client);
  info!("{}", user.email);
  let taken_user = user_collection.find_one(doc! {
    "email": &user.email
  }, None).await.unwrap();
  if taken_user.is_none() {
    return HttpResponse::Ok().json(ResponseMessage {
      success: 0,
      message: String::from("邮箱或密码错误！")
    })
  }
  let taken_user = taken_user.unwrap();
  let verified = verify_password(&user.password, &taken_user.password.as_str());
  if !verified {
    return HttpResponse::Ok().json(ResponseMessage {
      success: 0,
      message: String::from("邮箱或密码错误！")
    })
  }
  let token = Claims::generate_token(&taken_user._id.unwrap().to_string());
  HttpResponse::Ok().json(
    LoginMessage {
      success: 1,
      token
    }
  )
}