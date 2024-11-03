#![allow(non_snake_case)]
use crate::codegen::amusement_post::AmusementPost;
use crate::codegen::post::Comment;
use crate::db::models::NewAmusementPost;
use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use models::NullableIntArray;
use schema::Posts::{comments_id, people_all};
use serde::de::IntoDeserializer;

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
impl models::Comment {
    pub fn to_proto_comment(&self) -> Comment {
        let update_time = if let Some(update_naive_time) = self.updated_at {
            Some(update_naive_time.and_utc().timestamp())
        } else {
            None
        };
        let comment = Comment {
            id: self.id,
            user_id: self.user_id,
            post_id: self.post_id,
            content: self.content.clone(),
            likes: self.likes,
            created_at: self.created_at.and_utc().timestamp(),
            updated_at: update_time,
        };
        comment
    }
}

impl models::Post {
    pub fn from_proto_amusement_post(
        post: Option<AmusementPost>,
    ) -> Result<models::NewAmusementPost, Box<dyn StdError>> {
        if let Some(amusement_post) = post {
            // get amusement post field
            let the_people_all = Some(amusement_post.people_all);
            let the_people_already = Some(amusement_post.people_already);
            let the_game_type = Some(models::GameType::from_proto_type(
                &amusement_post.game_type(),
            ));
            let the_amuse_place = Some(amusement_post.amuse_place);
            let the_start_time = Some(NaiveDateTime::from_timestamp(amusement_post.start_time, 0));
            let the_contact = Some(amusement_post.contact);
            if let Some(base_post) = amusement_post.post {
                // store images
                let mut image_ids = vec![];
                for image in &base_post.images {
                    let image_id = add_image(image)?;
                    image_ids.push(Some(image_id));
                }

                // make post to insert
                let new_amusement_post = NewAmusementPost {
                    title: base_post.title,
                    user_id: base_post.user_id,
                    content: base_post.content,
                    post_type: crate::db::models::PostType::AMUSEMENTPOST,
                    images: NullableIntArray(image_ids),
                    comments_id: NullableIntArray(vec![]),

                    people_all: the_people_all,
                    people_already: the_people_already,
                    game_type: the_game_type,
                    amuse_place: the_amuse_place,
                    start_time: the_start_time,
                    contact: the_contact,
                };
                Ok(new_amusement_post)
            } else {
                Err(Box::new(std::fmt::Error))
            }
        } else {
            Err(Box::new(std::fmt::Error))
        }
    }

    // convert a models::Post to codegen::amusement_post::AmusementPost
    pub fn to_proto_amusement_post(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<AmusementPost, Box<dyn StdError>> {
        if self.post_type != models::PostType::AMUSEMENTPOST {
            return Err(Box::new(std::fmt::Error));
        }
        let update_time = if let Some(update_naive_time) = self.updated_at {
            Some(update_naive_time.and_utc().timestamp())
        } else {
            None
        };
        // get comments
        let mut comments = vec![];
        for comment_id in &self.comments_id.0 {
            if let Some(comment_id) = comment_id {
                let comment = query_comment_by_id(conn, *comment_id)?;
                comments.push(comment.to_proto_comment());
            }
        }

        // get images
        let mut images = vec![];
        for image_id in &self.images.0 {
            if let Some(image_id) = image_id {
                let image = query_image_by_id(*image_id)?;
                images.push(image);
            }
        }

        let base_post = crate::codegen::post::Post {
            id: self.id,
            title: self.title.clone(),
            user_id: self.user_id,
            content: self.content.clone(),
            likes: self.likes,
            favorates: self.favorates,
            created_at: self.created_at.and_utc().timestamp(),
            updated_at: update_time,
            comments: comments,
            images: images,
            post_type: self.post_type.to_proto_type().into(),
        };

        // get unique field of amusement post
        let the_people_all = self.people_all.ok_or_else(|| Box::new(std::fmt::Error))?;
        let the_people_already = self
            .people_already
            .ok_or_else(|| Box::new(std::fmt::Error))?;
        let the_game_type = self
            .game_type
            .as_ref()
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .to_proto_type()
            .into();
        let the_start_time = self
            .start_time
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .and_utc()
            .timestamp();
        let the_amuse_place = self
            .amuse_place
            .as_ref()
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .clone();
        let the_contact = self
            .contact
            .as_ref()
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .clone();

        let amusement_post = AmusementPost {
            post: Some(base_post),
            people_all: the_people_all,
            people_already: the_people_already,
            game_type: the_game_type,
            start_time: the_start_time,
            amuse_place: the_amuse_place,
            contact: the_contact,
        };
        Ok(amusement_post)
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
pub fn insert_amusement_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_post: &models::NewAmusementPost,
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

pub fn query_and_filter_amusement_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    the_game_type: models::GameType,
    the_people_all_lowbound: i32,
    the_people_all_upbound: i32,
    the_people_diff_upbound: i32,
    the_time_about: NaiveDateTime,
    limit: i32,
) -> Result<Vec<models::Post>, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let posts = Posts
        .filter(schema::Posts::post_type.eq(&models::PostType::AMUSEMENTPOST))
        .filter(schema::Posts::game_type.eq(&the_game_type))
        .filter(schema::Posts::people_all.ge(the_people_all_lowbound))
        .filter(schema::Posts::people_all.le(the_people_all_upbound))
        .filter(
            (schema::Posts::people_all - schema::Posts::people_already).le(the_people_diff_upbound),
        )
        .filter(schema::Posts::start_time.le(the_time_about + Duration::hours(2)))
        .filter(schema::Posts::start_time.ge(the_time_about - Duration::hours(2)))
        .limit(limit.into())
        .select(models::Post::as_select())
        .load(conn)
        .map_err(|e| e.to_string())?;
    Ok(posts)
}

pub fn insert_comment_and_update_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_comment: &models::NewComment,
) -> Result<(), Box<dyn StdError>> {
    use crate::dbschema::Comments::dsl::*;
    use crate::dbschema::Posts::dsl::*;

    let inserted_comment: models::Comment = diesel::insert_into(Comments)
        .values(new_comment)
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

pub fn query_post_by_user_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    the_user_id: i32,
) -> Result<Vec<models::Post>, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let posts = Posts
        .filter(schema::Posts::user_id.eq(&the_user_id))
        .select(models::Post::as_select())
        .load(conn)
        .map_err(|e| e.to_string())?;
    Ok(posts)
}

pub fn query_image_by_id(image_id: i32) -> Result<Vec<u8>, Box<dyn StdError>> {
    //TODO: read file from local filesystem.
    // Convert to bytes.
    todo!()
}

pub fn add_image(images: &Vec<u8>) -> Result<i32, Box<dyn StdError>> {
    //TODO: write file to local filesystem and return an id.

    todo!()
}

pub fn delete_image(image_id: i32) -> Result<(), Box<dyn StdError>> {
    //TODO: delete image from local filesystem
    todo!()
}
