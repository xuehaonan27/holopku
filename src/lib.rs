pub mod codegen {
    pub mod auth;
    pub mod forum;
    pub mod hello;
}

pub mod auth;
pub mod crypto;
pub mod db;
pub mod forum;
pub mod hello;
pub mod middleware;
use db::schema as dbschema;
use log::info;
use std::env;
use std::sync::LazyLock;

pub const AUTHORIZATION_KEY: &'static str = "Authorization";
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
const AES256KEY: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let key_str = env::var("AES256KEY").expect("Must set AES256KEY");
    let key_bytes = key_str.as_bytes();
    let mut key = [0u8; 32];
    let len = key_bytes.len().min(32);
    key[..len].copy_from_slice(&key_bytes[..len]);
    key
});
const AES256IV: LazyLock<[u8; 16]> = LazyLock::new(|| {
    let iv_str = env::var("AES256IV").expect("Must set AES256IV");
    let iv_bytes = iv_str.as_bytes();
    let mut iv = [0u8; 16];
    let len = iv_bytes.len().min(16);
    iv[..len].copy_from_slice(&iv_bytes[..len]);
    iv
});

/// Check all environment variables to assure integrity.
pub fn check_envs() {
    // log safe: information stored on server.
    let _env = JWT_SECRET;
    info!("JWT_SECRET={:?}", *JWT_SECRET);
    let _env = JWT_EXPIRE_TIME;
    info!("JWT_EXPIRE_TIME={:?}", *JWT_EXPIRE_TIME);
    let _env = JWT_ISSUER;
    info!("JWT_ISSUER={:?}", JWT_ISSUER);
    let _env = AES256KEY;
    info!("AES256KEY={:?}", *AES256KEY);
    let _env = AES256IV;
    info!("AES256IV={:?}", *AES256IV);
}
