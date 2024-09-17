//! HokoPKU authentication module.
pub mod iaaa;
pub mod password;

use iaaa::login_iaaa;
use log::{error, trace};
use password::{login_password, register_password};
use tonic::{Request, Response, Status};

use crate::codegen::auth::auth_server::Auth;
use crate::codegen::auth::{GetUserRequest, GetUserResponse};
use crate::codegen::auth::{LoginRequest, LoginResponse};
use crate::codegen::auth::{RegisterRequest, RegisterResponse};
use crate::db::DBClient;

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
}
