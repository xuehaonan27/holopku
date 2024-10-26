#![allow(non_snake_case)]
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use models::NullableIntArray;
use schema::Posts::comments_id;

use std::error::Error as StdError;

use crate::auth::iaaa::IAAAValidateResponse;

pub(crate) mod models;
pub(crate) mod schema;

#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error("Fail to connect database: {0}")]
    Connection(String),
    #[error("Fail to fetch connection: {0}")]
    FetchConn(String),
}

pub type DBResult<T> = std::result::Result<T, DBError>;

/// Database client. Since `PgPool` is clone-safe, `DBClient` is clone-safe as well.
#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DBClient {
    pub fn connect(database_url: &String) -> DBResult<Self> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .map_err(|e| DBError::Connection(e.to_string()))?;

        Ok(Self { pool })
    }

    pub fn get_conn(&self) -> DBResult<PooledConnection<ConnectionManager<PgConnection>>> {
        let conn = self
            .pool
            .get()
            .map_err(|e| DBError::FetchConn(e.to_string()))?;
        Ok(conn)
    }
}

pub fn get_iaaa_user_from_db(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    resp: IAAAValidateResponse,
) -> Result<models::User, Box<dyn StdError>> {
    use crate::dbschema::Users::dsl::*;
    let dbuser: Option<models::User> = Users
        .filter(schema::Users::username.eq(&resp.user_info.identity_id))
        .select(models::User::as_select())
        .first(conn)
        .ok();

    let dbuser = if let Some(dbuser) = dbuser {
        dbuser
    } else {
        // create a new user
        let new_user = models::IaaaNewUser {
            username: resp.user_info.identity_id,
            email: None, // FIXME: IAAA no email?
            login_provider: models::LoginProvider::IAAA,
            nickname: Some(resp.user_info.name),
        };
        let new_user: models::User = diesel::insert_into(schema::Users::table)
            .values(&new_user)
            .returning(models::User::as_returning())
            .get_result(conn)
            .map_err(|_| "Failed to create user")?;
        new_user
    };
    Ok(dbuser)
}

pub fn get_password_user_from_db(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    user_name: &String,
) -> Result<models::User, Box<dyn StdError>> {
    use crate::dbschema::Users::dsl::*;
    let dbuser: models::User = Users
        .filter(schema::Users::username.eq(&user_name))
        .select(models::User::as_select())
        .first(conn)
        .map_err(|e| e.to_string())?;
    Ok(dbuser)
}

pub fn insert_password_user_into_db(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_user: &models::PasswordNewUser,
) -> Result<models::User, Box<dyn StdError>> {
    use crate::dbschema::Users::dsl::*;
    let new_user = diesel::insert_into(Users)
        .values(new_user)
        .returning(models::User::as_returning())
        .get_result(conn)?;
    Ok(new_user)
}

pub fn insert_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_post: &models::Post,
) -> Result<models::Post, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let new_post = diesel::insert_into(Posts)
        .values(new_post)
        .returning(models::Post::as_returning())
        .get_result(conn)?;
    Ok(new_post)
}

pub fn delete_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    post_id: i32,
) -> Result<(), Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    diesel::delete(Posts.filter(id.eq(post_id))).execute(conn)?;
    Ok(())
}

pub fn query_post_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    post_id: i32,
) -> Result<models::Post, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let post: models::Post = Posts
        .filter(schema::Posts::id.eq(&post_id))
        .select(models::Post::as_select())
        .first(conn)
        .map_err(|e| e.to_string())?;
    Ok(post)
}
