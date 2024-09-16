use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tonic::{Request, Status};

/// Authentication interceptor to verify JWT in the request.
/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.

pub fn auth_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", request);

    let token = match request.metadata().get("authorization") {
        Some(token) => token.to_str().unwrap_or(""),
        None => return Err(Status::unauthenticated("Missing authorization header")),
    };

    // verify JWT
    if let Ok(claims) = validate_jwt(token) {
        // validate time
        let now = chrono::Utc::now().timestamp();
        let expire_at = claims.iat + claims.exp;
        if now >= (expire_at as i64) {
            return Err(Status::unauthenticated("Token expired"));
        }
        // communicate with Redis to check whether it's valid or not
        todo!("communicate with Redis and Database to check whether the user is valid or not");  
        Ok(request)
    } else {
        Err(Status::unauthenticated("Invalid token"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    // nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

const JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    let jwt_secret = std::env::var("JWT_SECRET").expect("Must set JWT_SECRET");
    jwt_secret
});
const JWT_EXPIRE_TIME: LazyLock<usize> = LazyLock::new(|| {
    let jwt_expire_time = std::env::var("JWT_EXPIRE_TIME").expect("Must set JWT_EXPIRE_TIME");
    jwt_expire_time
        .parse::<usize>()
        .expect("JWT_EXPIRE_TIME must be set to a positive integer")
});
const JWT_ISSUER: &'static str = "HoloPKU server";

pub fn issue_token(user_id: &String, email: &String) -> Result<String, JwtError> {
    let token = issue_token_inner(user_id, email)?;
    // encrypt the token
    todo!("encrypt the token")
}

fn issue_token_inner(user_id: &String, email: &String) -> Result<String, JwtError> {
    let claims = Claims {
        iss: JWT_ISSUER.into(),
        exp: *JWT_EXPIRE_TIME,
        iat: chrono::Utc::now().timestamp() as usize,
        aud: user_id.clone(),
        sub: email.clone(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}
