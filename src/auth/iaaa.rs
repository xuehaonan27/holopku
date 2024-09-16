use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use tonic::Status;

use crate::middleware::issue_token;
use crate::{codegen::auth::LoginResponse, db::get_iaaa_user_from_db};

// IAAA logic
const VALIDATE_ENDPOINT: &'static str = "https://iaaa.pku.edu.cn/iaaa/svc/token/validate.do";

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct IAAAUserInfo {
    // example 'Tom'
    #[serde(rename = "name")]
    pub name: String,

    // example: 'Kaitong'
    #[serde(rename = "status")]
    status: String,

    // example: '2200088888'
    #[serde(rename = "identityId")]
    pub identity_id: String,

    // example: '00048'
    #[serde(rename = "deptId")]
    dept_id: String,

    // example: '信息科学技术学院'
    #[serde(rename = "dept")]
    dept: String,

    // example: '学生'
    #[serde(rename = "identityType")]
    identity_type: String,

    // example: '本专科学生'
    #[serde(rename = "detailType")]
    detail_type: String,

    // example: '在校'
    #[serde(rename = "identityStatus")]
    identity_status: String,

    // example: '燕园'
    #[serde(rename = "campus")]
    campus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct IAAAValidateResponse {
    #[serde(rename = "success")]
    success: bool,
    #[serde(rename = "errCode")]
    err_code: String,
    #[serde(rename = "errMsg")]
    err_msg: String,
    #[serde(rename = "userInfo")]
    pub user_info: IAAAUserInfo,
}

impl IAAAValidateResponse {
    pub fn is_success(&self) -> bool {
        self.success
    }
}

pub(super) async fn login_iaaa(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    ip_address: &String,
    iaaa_id: &String,
    iaaa_key: &String,
    token: &String,
) -> Result<LoginResponse, Status> {
    let resp = if std::env::var("TEST")
        .and_then(|x| Ok(x.to_ascii_lowercase()))
        .is_ok_and(|x| x.eq("true"))
    {
        example_iaaa_validate_response()
    } else {
        validate(ip_address, iaaa_id, iaaa_key, token)
            .await
            .map_err(|_| Status::unauthenticated("Fail to validate"))?
    };

    if !resp.is_success() {
        return Err(Status::unauthenticated("Fail to authorize"));
    }

    let dbuser = get_iaaa_user_from_db(conn, resp)
        .await
        .map_err(|_| Status::unauthenticated("Fail to find user or auto-register for IAAA user"))?;

    let created_at: i64 = dbuser.created_at.and_utc().timestamp();
    let updated_at: Option<i64> = dbuser
        .updated_at
        .and_then(|x| Some(x.and_utc().timestamp()));

    let token = issue_token(
        &dbuser.id.to_string(),
        dbuser.email.as_ref().unwrap_or(&"".to_string()),
    )
    .map_err(|_| Status::unauthenticated("Fail to assign token"))?;

    use crate::codegen::auth::User;
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
}

pub async fn validate(
    remote_addr: &String,
    app_id: &String,
    app_key: &String,
    token: &String,
) -> Result<IAAAValidateResponse, Box<dyn StdError>> {
    let payload = format!("appId={app_id}&remoteAddr={remote_addr}&token={token}");
    let sign = md5_hash(&(payload.clone() + &app_key));
    let url = format!("{VALIDATE_ENDPOINT}?{payload}&msgAbs={sign}");
    let data = reqwest::get(url)
        .await?
        .json::<IAAAValidateResponse>()
        .await?;
    return Ok(data);
}

fn md5_hash(msg: &String) -> String {
    let digest = md5::compute(msg);
    format!("{:x}", digest)
}

fn example_iaaa_validate_response() -> IAAAValidateResponse {
    IAAAValidateResponse {
        success: true,
        err_code: "".into(),
        err_msg: "".into(),
        user_info: IAAAUserInfo {
            name: "Tom".into(),
            status: "Present".into(),
            identity_id: "2200088888".into(),
            dept_id: "example".into(),
            dept: "example".into(),
            identity_type: "example".into(),
            detail_type: "example".into(),
            identity_status: "example".into(),
            campus: "pku".into(),
        },
    }
}
