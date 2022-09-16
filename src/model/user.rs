use actix_web::{FromRequest, Error, HttpRequest, dev, error::ErrorBadRequest};
use std::future::{self, ready};
use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::Utc;
use jsonwebtoken::{Header, EncodingKey, decode, DecodingKey, Validation};
use std::{env, future::Ready};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Validate)]
pub struct User {
    pub _id: Option<ObjectId>,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub created_at: Option<DateTime>
}

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn generate_token(id: &str) -> String {
        let now = Utc::now().timestamp();

        let payload = Claims {
            exp: (now + ONE_WEEK) as usize,
            sub: id.to_string(),
        };
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref())).unwrap()
    }
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct AuthorizedUser {
    pub token: String,
    pub sub: String,
}

impl FromRequest for AuthorizedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.headers().get("Authorization") {
            Some(val) => match val.to_str() {
                Ok(v) => {
                    //
                    // let t = v.into();
                    let my_slice: Vec<&str> = v.split(" ").collect();
                    let k = env::var("JWT_SECRET").unwrap();

                    if my_slice.len() != 2 {
                        return future::ready(Err(ErrorBadRequest("Not Authorized")));
                    }

                    let token_claims_res =
                        decode::<Claims>(my_slice[1], &DecodingKey::from_secret(k.as_ref()), &Validation::default());

                    if token_claims_res.is_err() {
                        // dbg!("{:?}", token_claims_res);
                        return future::ready(Err(ErrorBadRequest("Not Authorized")));
                    }

                    let claims = token_claims_res.unwrap().claims;
                    let authorized_user = AuthorizedUser {
                        sub: claims.sub,
                        token: v.to_string()
                    };
                    ready(Ok(authorized_user))
                }
                Err(e) => future::ready(Err(ErrorBadRequest(e))),
            },
            None => future::ready(Err(ErrorBadRequest("Not Authorized")))
        }
    }
}