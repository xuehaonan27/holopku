use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use tonic::Status;

use crate::codegen::auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::db::models::{LoginProvider, PasswordNewUser};
use crate::db::{get_password_user_from_db, insert_password_user_into_db};
use crate::middleware::issue_token;

pub(super) async fn login_password(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    req: LoginRequest,
) -> Result<LoginResponse, Status> {
    let username = &req.username;
    let dbuser = get_password_user_from_db(conn, username)
        .await
        .map_err(|_| Status::not_found("No such user"))?;

    let hash = dbuser
        .password
        .as_ref()
        .ok_or_else(|| Status::internal("user without password"))?;

    if bcrypt::verify(&req.password, &hash).is_ok() {
        use crate::codegen::auth::User;
        let created_at: i64 = dbuser.created_at.and_utc().timestamp();
        let updated_at: Option<i64> = dbuser
            .updated_at
            .and_then(|x| Some(x.and_utc().timestamp()));
        let token = issue_token(
            &dbuser.id.to_string(),
            dbuser.email.as_ref().unwrap_or(&"".to_string()),
        )
        .map_err(|_| Status::unauthenticated("Fail to assign token"))?;
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
            }),
            token,
        };
        Ok(response)
    } else {
        Err(Status::unauthenticated("wrong password"))
    }
}

pub(super) async fn register_password(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    req: RegisterRequest,
) -> Result<RegisterResponse, Status> {
    let password = &req.password;
    let hashed_password =
        bcrypt::hash(&password, 10).map_err(|_| Status::internal("Fail to register new user"))?;

    if get_password_user_from_db(conn, &req.username).await.is_ok() {
        return Err(Status::unavailable("User exist"));
    }

    let new_user = PasswordNewUser {
        username: req.username,
        password: Some(hashed_password),
        login_provider: LoginProvider::PASSWORD,
        email: Some(req.email),
        nickname: "".into(),
    };

    let _ = insert_password_user_into_db(conn, new_user)
        .await
        .map_err(|_| Status::internal("Fail to register new user"))?;

    let response = RegisterResponse {
        success: true,
        message: "created success".into(),
    };

    Ok(response)
}
