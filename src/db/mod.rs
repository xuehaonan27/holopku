#![allow(non_snake_case)]
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use models::NullableIntArray;

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
) -> Result<models::Post, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    // 获取要删除的Post对象
    let post_to_delete: models::Post = Posts.filter(id.eq(post_id)).first(conn)?;
    // 删除Post
    diesel::delete(Posts.filter(id.eq(post_id))).execute(conn)?;
    Ok(post_to_delete)
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

pub fn query_comment_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    comment_id: i32,
) -> Result<models::Comment, Box<dyn StdError>> {
    use crate::dbschema::Comments::dsl::*;
    let comment = Comments
        .filter(schema::Comments::id.eq(&comment_id))
        .select(models::Comment::as_select())
        .first(conn)
        .map_err(|e| e.to_string())?;
    Ok(comment)
}

pub fn insert_comment_and_update_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_comment: models::Comment,
) -> Result<(), Box<dyn StdError>> {
    use crate::dbschema::Comments::dsl::*;
    use crate::dbschema::Posts::dsl::*;

    let inserted_comment: models::Comment = diesel::insert_into(Comments)
        .values(&new_comment)
        .get_result(conn)?;

    let new_comments_id = {
        let mut current_comments_id: NullableIntArray = Posts
            .filter(schema::Posts::id.eq(inserted_comment.post_id))
            .select(comments_id)
            .first(conn)?;

        current_comments_id.0.push(Some(inserted_comment.id));
        current_comments_id
    };

    diesel::update(Posts.filter(schema::Posts::id.eq(inserted_comment.post_id)))
        .set(comments_id.eq(new_comments_id))
        .execute(conn)?;

    Ok(())
}

pub fn delete_comment_and_update_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    comment_id: i32,
) -> Result<(), Box<dyn StdError>> {
    use crate::dbschema::Comments::dsl::*;
    use crate::dbschema::Posts::dsl::*;

    // 获取要删除的评论
    let comment_to_delete: models::Comment = Comments
        .filter(schema::Comments::id.eq(comment_id))
        .first(conn)?;

    // 删除评论
    diesel::delete(Comments.filter(schema::Comments::id.eq(comment_id))).execute(conn)?;

    // 更新Post的comments_id字段
    let new_comments_id = {
        let mut current_comments_id: NullableIntArray = Posts
            .filter(schema::Posts::id.eq(comment_to_delete.post_id))
            .select(comments_id)
            .first(conn)?;

        // 从comments_id数组中移除要删除的评论ID
        current_comments_id.0.retain(|&x| x != Some(comment_id));
        current_comments_id
    };

    diesel::update(Posts.filter(schema::Posts::id.eq(comment_to_delete.post_id)))
        .set(comments_id.eq(new_comments_id))
        .execute(conn)?;

    Ok(())
}

pub fn query_image_by_id(image_id: i32) -> Result<Vec<u8>, Box<dyn StdError>> {
    //TODO: read file from local filesystem.
    // Convert to bytes.
    todo!()
}

pub fn add_image(image_id: i32, images: &Vec<u8>) -> Result<(), Box<dyn StdError>> {
    //TODO: write file to local filesystem.

    todo!()
}

pub fn delete_image(image_id: i32) -> Result<(), Box<dyn StdError>> {
    //TODO: delete image from local filesystem
    todo!()
}
