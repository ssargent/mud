use axum::{
    body::Body,
    extract::{Json, Request, State},
    http,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    api::{ApiResponse, Payload},
    app_state::AppState,
    db::{system::NewUser, SystemUserRepository},
};
use diesel::{Connection, PgConnection};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult {
    pub token: String,
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogInData {
    pub email: String,
    pub password: String,
}

pub struct AuthError {
    message: String,
    status_code: StatusCode,
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
            "status": self.status_code.as_u16(),
        }));

        (self.status_code, body).into_response()
    }
}

pub fn encode_jwt(current_user: CurrentUser) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::days(1);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let sub = current_user.id.to_string();
    let email = current_user.email;

    let claims = Claims {
        exp,
        iat,
        email,
        sub,
    };
    let secret = jwt_token.clone();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "randomstring".to_string();
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn auth_login(
    State(state): State<AppState>,
    Json(user_data): Json<LogInData>,
) -> ApiResponse<LoginResult> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let user = match SystemUserRepository::get_by_email(&mut conn, user_data.email.as_str()) {
        Ok(Some(user)) => user,
        Ok(None) => return ApiResponse::Error("invalid credentials".to_string()),
        Err(_) => return ApiResponse::Error("Failed to get user".to_string()),
    };

    match verify_password(&user_data.password, &user.password) {
        Ok(true) => {}
        Ok(false) => return ApiResponse::Unauthorized("invalid Credentials".to_string()),
        Err(_) => return ApiResponse::Error("Failed to verify password".to_string()),
    }

    let cu = CurrentUser {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        password_hash: user.password,
    };

    match encode_jwt(cu) {
        Ok(token) => ApiResponse::JsonData(Payload {
            data: LoginResult {
                token: token.clone(),
                status: StatusCode::OK.as_u16(),
                message: "Login successful".to_string(),
            },
        }),
        Err(_) => ApiResponse::Error("Failed to create token".to_string()),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserResult {
    pub user_id: i64,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserData {
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: String,
}

pub async fn auth_register(
    State(state): State<AppState>,
    Json(new_user): Json<NewUserData>,
) -> ApiResponse<NewUserResult> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let hashed_password = match hash_password(&new_user.password) {
        Ok(hash) => hash,
        Err(e) => return ApiResponse::Error(e.to_string()),
    };

    let txn_result = conn.transaction::<_, diesel::result::Error, _>(|txn| {
        let new_user = NewUser {
            username: new_user.username,
            password: hashed_password,
            email: new_user.email,
            full_name: new_user.full_name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        match SystemUserRepository::get_by_email(txn, &new_user.email) {
            Ok(Some(_)) => Err(diesel::result::Error::RollbackTransaction),
            Ok(None) => {
                let created = match SystemUserRepository::create(txn, &new_user) {
                    Ok(user) => NewUserResult {
                        user_id: user.id,
                        email: user.email,
                    },
                    Err(e) => {
                        eprintln!("Error creating user: {:?}", e);
                        return Err(diesel::result::Error::RollbackTransaction);
                    }
                };

                Ok(created)
            }
            Err(e) => {
                eprintln!("Error creating user: {:?}", e);
                Err(diesel::result::Error::RollbackTransaction)
            }
        }
    });

    match txn_result {
        Ok(response) => ApiResponse::Created(Payload { data: response }),
        Err(err) => match err {
            diesel::result::Error::RollbackTransaction => {
                ApiResponse::BadRequest(vec!["User already exists".to_string()])
            }
            _ => ApiResponse::Error("Failed to create user".to_string()),
        },
    }
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub password_hash: String,
}

pub async fn authorize(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty Auth Header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the request in an Authorization header"
                    .to_string(),
                status_code: StatusCode::FORBIDDEN,
            })
        }
    };

    let mut header = auth_header.split_whitespace();
    let (bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Invalid JWT token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    let current_user = CurrentUser {
        id: token_data.claims.sub.parse().unwrap(),
        email: token_data.claims.email,
        full_name: "".to_string(),
        password_hash: "".to_string(),
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
