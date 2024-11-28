use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use log::{error, trace};
use tonic::Status;

use crate::codegen::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::db::models::{LoginProvider, PasswordNewUser};
use crate::db::{get_password_user_from_db, insert_password_user_into_db, query_image_by_id};
use crate::middleware::issue_token;

pub(super) async fn login_password(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    req: LoginRequest,
) -> Result<LoginResponse, Status> {
    let username = &req.username;
    let dbuser = get_password_user_from_db(conn, username).map_err(|e| {
        error!("User {username} not found: {e}");
        Status::not_found("No such user")
    })?;

    let hash = dbuser.password.as_ref().ok_or_else(|| {
        error!("User without password");
        Status::internal("user without password")
    })?;
    if let Ok(true) = bcrypt::verify(&req.password, &hash) {
        trace!("Password verified, issue token");
        use crate::codegen::auth::User;
        let created_at: i64 = dbuser.created_at.and_utc().timestamp();
        let updated_at: Option<i64> = dbuser
            .updated_at
            .and_then(|x| Some(x.and_utc().timestamp()));
        let token = issue_token(
            &dbuser.id.to_string(),
            dbuser.email.as_ref().unwrap_or(&"".to_string()),
        )
        .map_err(|e| {
            error!("Fail to issue token: {e}");
            Status::unauthenticated("Fail to assign token")
        })?;

        trace!("Issued token: {token:?}");

        let icon = query_image_by_id(dbuser.icon)
            .map_err(|_| Status::internal("Fail to get user icon"))?;

        let response = LoginResponse {
            success: true,
            user: Some(User {
                id: dbuser.id,
                username: dbuser.username,
                email: dbuser.email,
                login_provider: dbuser.login_provider as i32,
                nickname: dbuser.nickname,
                created_at,
                updated_at,
                icon,
                favorite_posts: dbuser.favorite_posts.to_vec_i32(),
                liked_posts: dbuser.liked_posts.to_vec_i32(),
                take_part_posts: dbuser.take_part_posts.to_vec_i32(),
            }),
            token,
        };
        Ok(response)
    } else {
        error!("Wrong password, user: {username}");
        Err(Status::unauthenticated("wrong password"))
    }
}

pub(super) async fn register_password(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    req: RegisterRequest,
) -> Result<RegisterResponse, Status> {
    let password = &req.password;
    let hashed_password = bcrypt::hash(&password, 10).map_err(|e| {
        error!("Fail to hash password: {e}");
        Status::internal("Fail to register new user")
    })?;

    if get_password_user_from_db(conn, &req.username).is_ok() {
        error!("User {} exist", req.username);
        return Err(Status::unavailable("User exist"));
    }

    let new_user = PasswordNewUser::new(req.username, Some(req.email), Some(hashed_password));

    let _ = insert_password_user_into_db(conn, &new_user).map_err(|e| {
        error!("Fail to register new user {new_user:#?}: {e}");
        Status::internal("Fail to register new user")
    })?;

    let response = RegisterResponse {
        success: true,
        message: "created success".into(),
    };

    Ok(response)
}
