use crate::dbschema::sql_types::LoginProvider as LoginProviderType;
use chrono::NaiveDateTime;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, ToSql};
use diesel::*;
use std::io::Write;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = LoginProviderType)]
pub enum LoginProvider {
    IAAA,
    PASSWORD,
}

impl ToSql<LoginProviderType, Pg> for LoginProvider {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            LoginProvider::IAAA => out.write_all(b"IAAA")?,
            LoginProvider::PASSWORD => out.write_all(b"PASSWORD")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<LoginProviderType, Pg> for LoginProvider {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"IAAA" => Ok(LoginProvider::IAAA),
            b"PASSWORD" => Ok(LoginProvider::PASSWORD),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = crate::dbschema::Users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub login_provider: LoginProvider,
    pub nickname: String,
    pub password: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::dbschema::Users)]
pub struct IaaaNewUser {
    pub username: String,
    pub email: Option<String>,
    pub login_provider: LoginProvider,
    pub nickname: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::dbschema::Users)]
pub struct PasswordNewUser {
    pub username: String,
    pub email: Option<String>,
    pub login_provider: LoginProvider,
    pub nickname: String,
    pub password: Option<String>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = crate::dbschema::Posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub content: String,
    pub likes: i32,
    pub favorates: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = crate::dbschema::Comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
    pub likes: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
