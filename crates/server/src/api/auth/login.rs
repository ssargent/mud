use std::fmt;
use std::num::NonZeroU32;

use axum::{
    Extension,
    body::Body,
    extract::{Json, Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use pbkdf2;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha3::{Digest, Sha3_512};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Payload},
    app_state::AppState,
    db::{
        PlayerEntitlementsRepository, SystemUserRepository, UserApiKeyRepository,
        player::Entitlement,
        system::{ActiveUserRole, NewUser, NewUserApiKey},
        system_schema::system::user_api_keys::key_type,
    },
};
use diesel::Connection;

use super::LoginResult;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub sub: String,
    pub roles: Vec<ActiveUserRole>,
    pub entitlements: Vec<String>,
    pub token_type: String,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogInData {
    pub email: String,
    pub password: String,
    pub token_type: Option<String>,
    pub enumerate_permissions: Option<bool>,
    pub enumerate_entitlements: Option<bool>,
}

const API_KEY_LEN: usize = 32; // 256 bits
const SECRET_KEY_LEN: usize = 64; // 512 bits
const DERIVED_KEY_LEN: usize = 64; // If using derived keys
const PBKDF2_ITERATIONS: u32 = 100_000;

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

pub fn gen_api_key(size: u8) -> String {
    let mut bytes = vec![0u8; size as usize];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut bytes);
    base32::encode(base32::Alphabet::Rfc4648Lower { padding: false }, &bytes)
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

/**
Encodes a JWT token with the provided user information, roles, entitlements, and token type.
### Arguments
* `current_user`: The current user information.
* `roles`: A vector of active user roles.
* `entitlements`: A vector of entitlements associated with the user.
* `token_type`: An optional string representing the type of token (e.g., "basic", "admin").
### Returns
 * `Result<String, StatusCode>`: The encoded JWT token if successful, or an error status code.
### Example
```rust
let current_user = CurrentUser {
    id: 1,
    email: "your.email@example.org".to_string(),
    full_name: "Your Name".to_string(),
    permissions: None,
};
let roles = vec![ActiveUserRole { id: 1, name: "user".to_string() }];
let entitlements = vec![Entitlement { code: "entitlement_code".to_string() }];
let token_type = Some("basic".to_string());
let jwt_token = encode_jwt(current_user, roles, entitlements, token_type);
match jwt_token {
    Ok(token) => println!("Encoded JWT token: {}", token),
    Err(status) => println!("Error encoding JWT token: {:?}", status),
}
```
### Errors
* Returns `StatusCode::INTERNAL_SERVER_ERROR` if the JWT token cannot be created.
### Note
* The JWT token is encoded using a secret key. Ensure that the secret key is kept secure and consistent across your application.
*/
pub fn encode_jwt(
    current_user: CurrentUser,
    roles: Vec<ActiveUserRole>,
    entitlements: Vec<Entitlement>,
    token_type: Option<String>,
) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::days(1);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let sub = current_user.id.to_string();
    let email = current_user.email;
    let requested_token_type = match token_type {
        Some(tt) => tt,
        None => "basic".to_string(),
    };

    let permission_list = current_user.permissions;

    let claims = Claims {
        exp,
        iat,
        email,
        sub,
        roles,
        entitlements: entitlements.iter().map(|e| e.code.clone()).collect(),
        token_type: requested_token_type,
        permissions: permission_list,
    };
    let secret = jwt_token.clone();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/**
Decodes a JWT token and returns the claims.
if the token is invalid, it returns a StatusCode::INTERNAL_SERVER_ERROR.
This function is used to verify the JWT token in the request.

### Arguments
* `token`: The JWT token to decode.
### Returns
 * `Result<TokenData<Claims>, StatusCode>`: The decoded claims if successful, or an error status code.

### Example
```rust
let token = "your_jwt_token_here".to_string();
let claims = decode_jwt(token);
match claims {
    Ok(data) => println!("Decoded claims: {:?}", data.claims),
    Err(status) => println!("Error decoding token: {:?}", status),
}
```
### Errors
* Returns `StatusCode::INTERNAL_SERVER_ERROR` if the token is invalid or cannot be decoded.
### Note
* Ensure that the secret used for encoding the JWT matches the one used for decoding.
* This function is typically used in middleware to authenticate requests by verifying the JWT token.
 */
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

/**
Handles user login by verifying credentials and returning a JWT token.
### Arguments
* `State(state)`: The application state containing the database connection pool.
* `Json(user_data)`: The user login data containing email and password.
### Returns
* `ApiResponse<LoginResult>`: A response containing the JWT token and status message if successful, or an error message if login fails.
*/
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

    let roles: Vec<ActiveUserRole> =
        match SystemUserRepository::get_active_user_roles(&mut conn, user.id) {
            Ok(roles) => roles,
            Err(_) => return ApiResponse::Error("Failed to get user roles".to_string()),
        };

    let entitlements: Vec<Entitlement> =
        match PlayerEntitlementsRepository::get_active_entitlements_by_user_id(&mut conn, user.id) {
            Ok(entitlements) => entitlements,
            Err(_) => return ApiResponse::Error("Failed to get user entitlements".to_string()),
        };

    //println!("entitlements_query: {:?}", entitlements_query);
    println!(
        "user_id_val: {:?}, Entitlements: {:?}",
        user.id, entitlements
    );

    let user_permissions = match user_data.enumerate_permissions {
        Some(true) => match SystemUserRepository::get_user_permissions(&mut conn, user.id) {
            Ok(permissions) => Some(permissions),
            Err(_) => return ApiResponse::Error("Failed to get user permissions".to_string()),
        },
        _ => None,
    };

    let cu = CurrentUser {
        id: user.id,
        email: user.email,
        full_name: user.full_name,
        permissions: user_permissions,
    };

    let token_type = match user_data.token_type.as_deref() {
        Some("basic") => "basic".to_string(),
        Some("admin") => "admin".to_string(),
        _ => "basic".to_string(), // Default to basic if not specified
    };

    match encode_jwt(cu, roles, entitlements, Some(token_type)) {
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
pub enum KeyType {
    Client,
    Server,
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Client => write!(f, "Client"),
            KeyType::Server => write!(f, "Server"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewApiKeyRequest {
    pub key_type: KeyType,
    pub expiration: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewApiKeyResult {
    pub id: i64,
    pub api_key: String,
    pub private_key: Option<String>,
    pub key_type: KeyType,
    pub expiration: Option<chrono::NaiveDateTime>,
}

/**
Creates a new API key for the current user.
### Arguments
* `Extension(current_user)`: The current user making the request.
* `State(state)`: The application state containing the database connection pool.
* `Json(api_key_request)`: The request body containing the API key type and expiration date.
### Returns
* `ApiResponse<NewApiKeyResult>`: A response containing the newly created API key details if successful, or an error message if creation fails.
 */
pub async fn auth_create_api_key(
    Extension(current_user): Extension<CurrentUser>,
    State(state): State<AppState>,
    Json(api_key_request): Json<NewApiKeyRequest>,
) -> ApiResponse<NewApiKeyResult> {
    let pool = state.db_pool.clone();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return ApiResponse::Error("Failed to get connection".to_string()),
    };

    let user = match SystemUserRepository::get_by_id(&mut conn, current_user.id) {
        Ok(user) => user,
        Err(_) => return ApiResponse::Error("Failed to get user".to_string()),
    };

    //let api_key_val = gen_api_key(36);
    let api_key_val = gen_enhanced_api_key(&user.username, user.id, "v1");
    let new_api_key = NewUserApiKey {
        user_id: user.id,
        api_key: api_key_val.clone(),
        key_type: api_key_request.key_type.to_string(),
        private_key: match api_key_request.key_type {
            KeyType::Client => None,
            KeyType::Server => Some(gen_api_key(SECRET_KEY_LEN as u8)),
        },
        expiration: api_key_request.expiration,
        created_at: Utc::now().naive_utc(),
        created_by: user.email.clone(),
        updated_at: Utc::now().naive_utc(),
        updated_by: user.email.clone(),
    };

    match UserApiKeyRepository::create(&mut conn, &new_api_key) {
        Ok(api_key) => ApiResponse::JsonData(Payload {
            data: NewApiKeyResult {
                id: api_key.id,
                api_key: api_key.api_key,
                private_key: api_key.private_key,
                key_type: match api_key.key_type.as_str() {
                    "Client" => KeyType::Client,
                    "Server" => KeyType::Server,
                    _ => KeyType::Client,
                },
                expiration: api_key.expiration,
            },
        }),
        Err(_) => ApiResponse::Error("Failed to create API key".to_string()),
    }
}

/**
Generates an enhanced API key using the provided username, ID, and version
The API key is generated by hashing the ID and a UUID, and then encoding it in base32.
The resulting API key is formatted with the username and version.

### Arguments
 * `username`: The username of the user for whom the API key is being generated.
 * `id`: The ID of the user for whom the API key is being generated.
 * `ver`: The version of the API key.

### Returns
  * `String`: The generated API key.
 */
fn gen_enhanced_api_key(username: &str, id: i64, ver: &str) -> String {
    // create a UUID type 4
    let secret_key_val = gen_api_key(API_KEY_LEN as u8);
    let api_key = format!(
        "{}_{}{}{}",
        format_username_for_api_key(username),
        ver,
        secret_key_val,
        id
    );
    api_key
}

fn format_username_for_api_key(username: &str) -> String {
    username.to_lowercase().replace(" ", "_").replace(".", "_")
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

/**
Registers a new user by creating a new user record in the database.
### Arguments
* `State(state)`: The application state containing the database connection pool.
* `Json(new_user)`: The request body containing the new user's data (username, password, email, full name).
### Returns
* `ApiResponse<NewUserResult>`: A response containing the newly created user's ID and email if successful, or an error message if registration fails.
 */
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
    pub permissions: Option<Vec<String>>,
}

/**
Middleware function to authorize requests by checking the JWT token in the Authorization header.
### Arguments
* `mut req`: The incoming request containing the Authorization header.
* `next`: The next middleware or handler to call if authorization is successful.
### Returns
* `Result<Response<Body>, AuthError>`: A response containing the next handler's response if authorization is successful, or an `AuthError` if authorization fails.
### Example
```rust
use axum::{Router, middleware::from_fn};
use crate::api::auth::authorize;
let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(from_fn(authorize));
```
### Errors
* Returns `AuthError` with a message and status code if the Authorization header is missing, empty, or if the JWT token is invalid.
### Note
* This middleware should be applied to routes that require authentication.
*/
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
            });
        }
    };

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Invalid JWT token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    let current_user = CurrentUser {
        id: token_data.claims.sub.parse().unwrap(),
        email: token_data.claims.email,
        full_name: "".to_string(),
        permissions: None,
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
