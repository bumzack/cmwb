use crate::user::user_by_username;
use crate::CONFIG;

use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum::RequestPartsExt;
use axum::{http::StatusCode, Json};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use common::db::verify_password;
use common::models::DbUser;
use jsonwebtoken::{decode, Validation};
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let extract = parts.extract::<TypedHeader<Authorization<Bearer>>>();
        let TypedHeader(Authorization(bearer)) =
            extract.await.map_err(|_| AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &CONFIG.keys.decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub async fn login(
    Json(payload): Json<AuthPayload>,
    //  State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<AuthBody>, (StatusCode, String)> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "missing credentials".to_string(),
        ));
    }

    let un = payload.client_id.clone();
    // let user = user_by_username(pool, &un).await;
    let user = DbUser {
        id: 12,
        username: "bla".to_string(),
        password: "bla".to_string(),
        created: Utc::now().naive_utc(),
        updated: Utc::now().naive_utc(),
    };
    let user:Result<Option<DbUser>, (StatusCode, String)> = Ok(Some(user));

    tracing::info!("got a user {:?}", &user);

    match user {
        Ok(u) => {
            // Here you can check the user credentials from a database
            let u = u.expect("should be a user there");
            if !verify_password(&u.password, &payload.client_secret) {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "an error occured - user not found".to_string(),
                ));
            }

            let claims = Claims {
                company: "cmwb".to_owned(),
                // Mandatory expiry time as UTC timestamp
                exp: 2000000000, // May 2033
                username: payload.client_id.clone(),
            };

            // Create the authorization token
            let token = encode(&Header::default(), &claims, &CONFIG.keys.encoding)
                // .map_err(|_| Err((
                //     StatusCode::INTERNAL_SERVER_ERROR,
                //     "cant create token".to_string(),
                // )))
                .expect("should could create a token");

            // Send the authorized token
            Ok(Json(AuthBody::new(token)))
        }
        Err(e) => {
            tracing::error!("an erro occurred {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "an error occured - but nobody knows what".to_string(),
            ))
        }
    }
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "username: {} company: {}, exp {}",
            self.username, self.company, self.exp
        )
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    company: String,
    exp: usize,
    username: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
