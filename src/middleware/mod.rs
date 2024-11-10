use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::trace;
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

use crate::crypto::{decrypt_aes256, encrypt_aes256};
use crate::{AUTHORIZATION_KEY, JWT_EXPIRE_TIME, JWT_ISSUER, JWT_SECRET};

/// Authentication interceptor to verify JWT in the request.
/// This function will get called on each inbound request, if a `Status`
/// is returned, it will cancel the request and return that status to the
/// client.

pub fn auth_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    trace!("Auth intercepting request: {:?}", request);

    /*
    // ignoring auth, when testing
    Ok(request)
    */

    // it's binary so use get_bin
    let token = match request.metadata().get_bin(AUTHORIZATION_KEY) {
        Some(token) => token,
        None => return Err(Status::unauthenticated("Missing authorization header")),
    };
    let tokenvec: Vec<u8> = token.to_bytes().unwrap().into();
    // can see that it's the same as token sent out
    println!("tokenvec is {:?}", tokenvec);
    // decode JWT
    let token = decrypt_aes256(&token.to_bytes().unwrap_or(prost::bytes::Bytes::new()))
        .map_err(|e| Status::unauthenticated(format!("Fail to decode token: {e}")))?;
    // [`std::mem::transmute`] no panic
    // TODO: change `jwt` crate [`jsonwebtoken::decoding::decode`] API to `decode_bytes` or something
    // when this API is available.

    let token = unsafe { std::str::from_utf8_unchecked(&token) };

    // verify JWT
    if let Ok(claims) = validate_jwt(token) {
        // validate time
        let now = chrono::Utc::now().timestamp();
        let expire_at = claims.iat + claims.exp;
        if now >= (expire_at as i64) {
            trace!("Token {token} expired");
            return Err(Status::unauthenticated("Token expired"));
        }
        // communicate with Redis to check whether it's valid or not
        // todo!("communicate with Redis and Database to check whether the user is valid or not");
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

pub fn issue_token(user_id: &String, email: &String) -> Result<Vec<u8>, JwtError> {
    let token = issue_token_inner(user_id, email)?;
    trace!("Token: {token}");
    // encrypt the token
    let encrypt_token = encrypt_aes256(token.as_bytes());
    trace!("Encrypted token: {token}");
    Ok(encrypt_token)
}

fn issue_token_inner(user_id: &String, email: &String) -> Result<String, JwtError> {
    let claims = Claims {
        iss: JWT_ISSUER.into(),
        exp: *JWT_EXPIRE_TIME,
        iat: chrono::Utc::now().timestamp() as usize,
        aud: user_id.clone(),
        sub: email.clone(),
    };
    trace!("Claims: {claims:#?}");
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
