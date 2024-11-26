//! HokoPKU authentication module.
pub mod iaaa;
pub mod password;

use iaaa::login_iaaa;
use log::{error, trace};
use password::{login_password, register_password};
use tonic::{Request, Response, Status};

use crate::codegen::auth::auth_server::Auth;
use crate::codegen::auth::{ChangeIconRequest, ChangeIconResponse};
use crate::codegen::auth::{ChangeUsernameRequest, ChangeUsernameResponse};
use crate::codegen::auth::{GetUserRequest, GetUserResponse};
use crate::codegen::auth::{LoginRequest, LoginResponse};
use crate::codegen::auth::{RegisterRequest, RegisterResponse};
use crate::db::{add_image, query_image_by_id, update_user_icon_id, update_username, DBClient};

#[derive(Debug)]
pub struct AuthService {
    pub client: DBClient,
    pub iaaa_id: String,
    pub iaaa_key: String,
}

#[tonic::async_trait]
impl Auth for AuthService {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        trace!("Login got request: {req:#?}");

        use crate::codegen::auth::LoginProvider;
        if req.auth_provider == LoginProvider::Iaaa as i32 {
            if req.iaaa_token.is_empty() {
                return Err(Status::unauthenticated("IAAA token cannot be empty!"));
            }
            if req.ip_address.is_none() {
                return Err(Status::unauthenticated("ip_address cannot be empty!"));
            }
        };

        // get connection to database
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to authorize")
        })?;
        let response = if req.auth_provider == LoginProvider::Iaaa as i32 {
            let token = &req.iaaa_token;
            let ip_address = req.ip_address.as_ref().unwrap(); // unwrap safe
            login_iaaa(conn, ip_address, &self.iaaa_id, &self.iaaa_key, &token).await
        } else if req.auth_provider == LoginProvider::Password as i32 {
            login_password(conn, req).await
        } else {
            error!("Unknown login provider: {}", req.auth_provider);
            Err(Status::invalid_argument("invalid login provider"))
        }?;

        Ok(Response::new(response))
    }

    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        use crate::codegen::auth::LoginProvider;
        let req = request.into_inner();
        trace!("Register got request: {req:#?}");

        let resp = if req.auth_provider == LoginProvider::Iaaa as i32 {
            Err(Status::unavailable("IAAA should not call Register"))
        } else if req.auth_provider == LoginProvider::Password as i32 {
            let conn = &mut self.client.get_conn().map_err(|e| {
                error!("Fail to get connection to database: {e}");
                Status::internal("Fail to authorize")
            })?;
            register_password(conn, req).await
        } else {
            Err(Status::invalid_argument("invalid login provider"))
        }?;

        Ok(Response::new(resp))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        trace!("GetUser got request: {req:#?}");
        todo!()
    }

    async fn change_icon(
        &self,
        request: tonic::Request<ChangeIconRequest>,
    ) -> Result<tonic::Response<ChangeIconResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Register got request: {req:#?}");
        let icon_bytes = req.new_icon;

        let image_id = add_image(&icon_bytes).map_err(|e| {
            error!("Fail to add icon: {e}");
            Status::internal("Fail to change icon")
        })?;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to authorize")
        })?;

        let Ok(dbuser) = update_user_icon_id(conn, req.user_id, image_id) else {
            error!("Fail to change icon");
            return Err(Status::internal("Fail to change icon"));
        };

        let created_at: i64 = dbuser.created_at.and_utc().timestamp();
        let updated_at: Option<i64> = dbuser
            .updated_at
            .and_then(|x| Some(x.and_utc().timestamp()));

        use crate::codegen::auth::User;
        let user = Some(User {
            id: dbuser.id,
            username: dbuser.username,
            email: dbuser.email,
            login_provider: dbuser.login_provider as i32,
            nickname: dbuser.nickname,
            created_at,
            updated_at,
            icon: icon_bytes,
            favorite_posts: dbuser.favorite_posts.to_vec_i32(),
            liked_posts: dbuser.liked_posts.to_vec_i32(),
            take_part_posts: dbuser.take_part_posts.to_vec_i32(),
        });

        Ok(Response::new(ChangeIconResponse {
            success: true,
            user,
        }))
    }

    async fn change_username(
        &self,
        request: tonic::Request<ChangeUsernameRequest>,
    ) -> Result<tonic::Response<ChangeUsernameResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Register got request: {req:#?}");
        let new_name = req.new_name;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to authorize")
        })?;

        let Ok(dbuser) = update_username(conn, req.user_id, new_name) else {
            error!("Fail to change username");
            return Err(Status::internal("Fail to change username"));
        };

        let created_at: i64 = dbuser.created_at.and_utc().timestamp();
        let updated_at: Option<i64> = dbuser
            .updated_at
            .and_then(|x| Some(x.and_utc().timestamp()));

        let icon = query_image_by_id(dbuser.icon).map_err(|e| {
            error!("Fail to query image by id {} :{e}", dbuser.icon);
            Status::internal("Fail to change username")
        })?;

        use crate::codegen::auth::User;
        let user = Some(User {
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
        });

        Ok(Response::new(ChangeUsernameResponse {
            success: true,
            user,
        }))
    }
}
