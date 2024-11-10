#![allow(non_snake_case)]
use crate::codegen::amusement_post::AmusementPost;
use crate::codegen::food_post::FoodPost;
use crate::codegen::post::Comment;
use crate::codegen::sell_post::SellPost;
use crate::db::models::NewAmusementPost;
use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use models::{NewFoodPost, NewSellPost, NullableIntArray};
use rand::Rng;
use schema::Posts::{comments_id, people_all};
use serde::de::IntoDeserializer;

use std::error::Error as StdError;
use std::io::{Read, Write};
use std::option;

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
    pub fn from_proto_sell_post(
        post: Option<SellPost>,
    ) -> Result<models::NewSellPost, Box<dyn StdError>> {
        if let Some(sell_post) = post {
            // get sell post field
            let the_contact = sell_post.contact.clone();
            let the_price = sell_post.price;
            let the_goods_type = models::GoodsType::from_proto_type(&sell_post.goods_type());

            if let Some(base_post) = sell_post.post {
                // store images
                let mut image_ids = vec![];
                for image in &base_post.images {
                    let image_id = add_image(image)?;
                    image_ids.push(Some(image_id));
                }

                // make post to insert
                let new_sell_post = NewSellPost {
                    title: base_post.title,
                    user_id: base_post.user_id,
                    content: base_post.content,
                    post_type: crate::db::models::PostType::SELLPOST,
                    images: NullableIntArray(image_ids),
                    comments_id: NullableIntArray(vec![]),
                    contact: the_contact,

                    price: Some(the_price),
                    goods_type: Some(the_goods_type),
                    sold: Some(false),
                };
                Ok(new_sell_post)
            } else {
                Err(Box::new(std::fmt::Error))
            }
        } else {
            Err(Box::new(std::fmt::Error))
        }
    }

    pub fn to_proto_sell_post(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<SellPost, Box<dyn StdError>> {
        if self.post_type != models::PostType::SELLPOST {
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

        // get unique field of sell post
        let the_contact = self.contact.clone();
        let the_price = self.price.ok_or_else(|| Box::new(std::fmt::Error))?;
        let the_goods_type = self
            .goods_type
            .as_ref()
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .to_proto_type();
        let if_sold = self.sold.ok_or_else(|| Box::new(std::fmt::Error))?;

        let sell_post = SellPost {
            post: Some(base_post),

            contact: the_contact,
            price: the_price,
            goods_type: the_goods_type.into(),
            sold: if_sold,
        };
        Ok(sell_post)
    }

    pub fn from_proto_food_post(
        post: Option<FoodPost>,
    ) -> Result<models::NewFoodPost, Box<dyn StdError>> {
        if let Some(food_post) = post {
            // get food post field
            let the_food_place = models::Place::from_proto_type(&food_post.food_place());
            let the_score = food_post.score;
            if let Some(base_post) = food_post.post {
                // store images
                let mut image_ids = vec![];
                for image in &base_post.images {
                    let image_id = add_image(image)?;
                    image_ids.push(Some(image_id));
                }

                // make post to insert
                let new_food_post = NewFoodPost {
                    title: base_post.title,
                    user_id: base_post.user_id,
                    content: base_post.content,
                    post_type: crate::db::models::PostType::FOODPOST,
                    images: NullableIntArray(image_ids),
                    comments_id: NullableIntArray(vec![]),

                    food_place: Some(the_food_place),
                    score: Some(the_score),
                };
                Ok(new_food_post)
            } else {
                Err(Box::new(std::fmt::Error))
            }
        } else {
            Err(Box::new(std::fmt::Error))
        }
    }

    pub fn to_proto_food_post(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<FoodPost, Box<dyn StdError>> {
        if self.post_type != models::PostType::FOODPOST {
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

        // get unique field of food post
        let the_food_place = self
            .food_place
            .as_ref()
            .ok_or_else(|| Box::new(std::fmt::Error))?
            .to_proto_type();
        let the_score = self.score.ok_or_else(|| Box::new(std::fmt::Error))?;

        let food_post = FoodPost {
            post: Some(base_post),

            food_place: the_food_place.into(),
            score: the_score,
        };
        Ok(food_post)
    }

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

pub fn insert_food_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_post: &models::NewFoodPost,
) -> Result<models::Post, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let new_post = diesel::insert_into(Posts)
        .values(new_post)
        .returning(models::Post::as_returning())
        .get_result(conn)?;
    Ok(new_post)
}

pub fn insert_sell_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    new_post: &models::NewSellPost,
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
    the_game_type: Option<models::GameType>,
    the_people_all_lowbound: i32,
    the_people_all_upbound: i32,
    the_people_diff_upbound: i32,
    the_time_about: Option<NaiveDateTime>,
    limit: i32,
) -> Result<Vec<models::Post>, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let posts = Posts
        .filter(schema::Posts::post_type.eq(&models::PostType::AMUSEMENTPOST))
        .filter(schema::Posts::people_all.ge(the_people_all_lowbound))
        .filter(schema::Posts::people_all.le(the_people_all_upbound))
        .filter(
            (schema::Posts::people_all - schema::Posts::people_already).le(the_people_diff_upbound),
        );
    if the_game_type.is_some() {
        let posts = posts.filter(schema::Posts::game_type.eq(the_game_type.unwrap())); // safe unwrap
        if the_time_about.is_some() {
            let posts = posts
                .filter(schema::Posts::start_time.le(the_time_about.unwrap() + Duration::hours(2)))
                .filter(schema::Posts::start_time.ge(the_time_about.unwrap() - Duration::hours(2)))
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        } else {
            let posts = posts
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        }
    } else {
        if the_time_about.is_some() {
            let posts = posts
                .filter(schema::Posts::start_time.le(the_time_about.unwrap() + Duration::hours(2)))
                .filter(schema::Posts::start_time.ge(the_time_about.unwrap() - Duration::hours(2)))
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        } else {
            let posts = posts
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        }
    }
}

pub fn query_and_filter_food_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    the_food_place: Option<models::Place>,
    the_score_lowbound: i32,
    is_random: bool,
    limit: i32,
) -> Result<Vec<models::Post>, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    if is_random {
        let mut the_posts: Vec<models::Post> = Posts
            .filter(schema::Posts::post_type.eq(&models::PostType::FOODPOST))
            .limit(limit.into())
            .select(models::Post::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;
        let the_random_one = the_posts.remove(rand::thread_rng().gen::<usize>() % the_posts.len());
        let posts = vec![the_random_one];
        Ok(posts)
    } else {
        let posts = Posts
            .filter(schema::Posts::post_type.eq(&models::PostType::FOODPOST))
            .filter(schema::Posts::score.ge(the_score_lowbound));

        if the_food_place.is_some() {
            let posts = posts
                .filter(schema::Posts::food_place.eq(the_food_place.unwrap())) //safe unwrap
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        } else {
            let posts = posts
                .limit(limit.into())
                .select(models::Post::as_select())
                .load(conn)
                .map_err(|e| e.to_string())?;
            Ok(posts)
        }
    }
}

pub fn query_and_filter_sell_post(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    the_goods_type: Option<models::GoodsType>,
    price_upbound: i32,
    limit: i32,
) -> Result<Vec<models::Post>, Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    let the_post = Posts
        .filter(schema::Posts::post_type.eq(&models::PostType::SELLPOST))
        .filter(schema::Posts::price.le(price_upbound))
        .filter(schema::Posts::sold.eq(false));
    if the_goods_type.is_some() {
        let the_post = the_post
            .filter(schema::Posts::goods_type.eq(&the_goods_type.unwrap()))
            .limit(limit.into())
            .select(models::Post::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;
        Ok(the_post)
    } else {
        let the_post = the_post
            .limit(limit.into())
            .select(models::Post::as_select())
            .load(conn)
            .map_err(|e| e.to_string())?;

        Ok(the_post)
    }
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

pub fn set_sold_for_sell_post_by_id(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    the_post_id: i32,
) -> Result<(), Box<dyn StdError>> {
    use crate::dbschema::Posts::dsl::*;
    diesel::update(Posts.filter(schema::Posts::id.eq(the_post_id)))
        .set(sold.eq(true))
        .execute(conn)?;
    Ok(())
}

pub fn query_image_by_id(image_id: i32) -> Result<Vec<u8>, Box<dyn StdError>> {
    //TODO: read file from local filesystem.
    // Convert to bytes.

    // most naive implement
    // need more improve
    let path = String::from("picture/") + &image_id.to_string();
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    // 读取文件内容到 buffer 中
    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

static mut IMAGE_ID: i32 = 0;
pub fn add_image(images: &Vec<u8>) -> Result<i32, Box<dyn StdError>> {
    //TODO: write file to local filesystem and return an id.

    // unsafe块线程不安全
    // 临时的存储方式，需要改进
    unsafe {
        let path = String::from("picture/") + &IMAGE_ID.to_string();
        let mut file = std::fs::File::create(path)?;
        file.write_all(images)?;
        IMAGE_ID += 1;
        Ok(IMAGE_ID - 1)
    }
}

pub fn delete_image(image_id: i32) -> Result<(), Box<dyn StdError>> {
    //TODO: delete image from local filesystem

    // need improve
    let path = String::from("picture/") + &image_id.to_string();
    std::fs::remove_file(path)?;

    Ok(())
}
