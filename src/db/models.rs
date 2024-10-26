use crate::dbschema::sql_types::GameType as GameTypeSql;
use crate::dbschema::sql_types::GoodsType as GoodsTypeSql;
use crate::dbschema::sql_types::LoginProvider as LoginProviderType;
use crate::dbschema::sql_types::Place as PlaceTypeSql;
use crate::dbschema::sql_types::PostType as PostTypeSql;
use chrono::NaiveDateTime;
use deserialize::FromSqlRow;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{IsNull, ToSql};
use diesel::sql_types::{Array, Nullable};
use diesel::*;
use sql_types::Integer;
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

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, AsChangeset, Insertable)]
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
    pub comments_id: NullableIntArray,
    pub images: NullableIntArray,
    pub post_type: PostType,
    pub contact: Option<String>,

    pub food_place: Option<Place>,
    pub score: Option<i32>,

    pub people_all: Option<i32>,
    pub people_already: Option<i32>,
    pub game_type: Option<GameType>,
    pub start_time: Option<NaiveDateTime>,
    pub amuse_place: Option<String>,

    pub price: Option<i32>,
    pub goods_type: Option<GoodsType>,
    pub sold: Option<bool>,
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Selectable, AsChangeset, Insertable)]
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

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = PostTypeSql)]
pub enum PostType {
    FOODPOST,
    SELLPOST,
    AMUSEMENTPOST,
}

impl ToSql<PostTypeSql, Pg> for PostType {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match *self {
            PostType::FOODPOST => out.write_all(b"FOODPOST")?,
            PostType::AMUSEMENTPOST => out.write_all(b"AMUSEMENTPOST")?,
            PostType::SELLPOST => out.write_all(b"SELLPOST")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PostTypeSql, Pg> for PostType {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"FOODPOST" => Ok(PostType::FOODPOST),
            b"AMUSEMENTPOST" => Ok(PostType::AMUSEMENTPOST),
            b"SELLPOST" => Ok(PostType::SELLPOST),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = PlaceTypeSql)]
pub enum Place {
    JiaYuan,
    YiYuan,
    ShaoYuan,
    YanNan,
    NongYuan,
    XueYi,
    XueWu,
    Other,
}

impl ToSql<PlaceTypeSql, Pg> for Place {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            Place::JiaYuan => out.write_all(b"JiaYuan")?,
            Place::YiYuan => out.write_all(b"YiYuan")?,
            Place::ShaoYuan => out.write_all(b"ShaoYuan")?,
            Place::YanNan => out.write_all(b"YanNan")?,
            Place::NongYuan => out.write_all(b"NongYuan")?,
            Place::XueYi => out.write_all(b"XueYi")?,
            Place::XueWu => out.write_all(b"XueWu")?,
            Place::Other => out.write_all(b"Other")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<PlaceTypeSql, Pg> for Place {
    fn from_sql(bytes: <Pg as backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"JiaYuan" => Ok(Place::JiaYuan),
            b"YiYuan" => Ok(Place::YiYuan),
            b"ShaoYuan" => Ok(Place::ShaoYuan),
            b"YanNan" => Ok(Place::YanNan),
            b"NongYuan" => Ok(Place::NongYuan),
            b"XueYi" => Ok(Place::XueYi),
            b"XueWu" => Ok(Place::XueWu),
            b"Other" => Ok(Place::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = GameTypeSql)]

pub enum GameType {
    WolfKill,
    JvBen,
    BloodTower,
    Karaok,
    BoardGame,
    Sports,
    Riding,
    Other,
}

impl ToSql<GameTypeSql, Pg> for GameType {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            GameType::WolfKill => out.write_all(b"WolfKill")?,
            GameType::JvBen => out.write_all(b"JvBen")?,
            GameType::BloodTower => out.write_all(b"BloodTower")?,
            GameType::Karaok => out.write_all(b"Karaok")?,
            GameType::BoardGame => out.write_all(b"BoardGame")?,
            GameType::Sports => out.write_all(b"Sports")?,
            GameType::Riding => out.write_all(b"Riding")?,
            GameType::Other => out.write_all(b"Other")?,
        };
        Ok(IsNull::No)
    }
}

impl FromSql<GameTypeSql, Pg> for GameType {
    fn from_sql(bytes: <Pg as backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"WolfKill" => Ok(GameType::WolfKill),
            b"JvBen" => Ok(GameType::JvBen),
            b"BloodTower" => Ok(GameType::BloodTower),
            b"Karaok" => Ok(GameType::Karaok),
            b"BoardGame" => Ok(GameType::BoardGame),
            b"Sports" => Ok(GameType::Sports),
            b"Riding" => Ok(GameType::Riding),
            b"Other" => Ok(GameType::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Eq)]
#[diesel(sql_type = GoodsTypeSql)]

pub enum GoodsType {
    Ticket,
    Book,
    Display,
    Computer,
    Other,
}

impl ToSql<GoodsTypeSql, Pg> for GoodsType {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            GoodsType::Book => out.write_all(b"Book")?,
            GoodsType::Computer => out.write_all(b"Computer")?,
            GoodsType::Display => out.write_all(b"Display")?,
            GoodsType::Ticket => out.write_all(b"Ticket")?,
            GoodsType::Other => out.write_all(b"Other")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<GoodsTypeSql, Pg> for GoodsType {
    fn from_sql(bytes: <Pg as backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Book" => Ok(GoodsType::Book),
            b"Computer" => Ok(GoodsType::Computer),
            b"Display" => Ok(GoodsType::Display),
            b"Ticket" => Ok(GoodsType::Ticket),
            b"Other" => Ok(GoodsType::Other),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Array<Integer>)]
pub struct IntArray(pub Vec<i32>);

impl FromSql<Array<Integer>, Pg> for IntArray {
    fn from_sql(bytes: <Pg as backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let vec = Vec::<i32>::from_sql(bytes)?;
        Ok(IntArray(vec))
    }
}

impl ToSql<Array<Integer>, Pg> for IntArray {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let vec: &Vec<i32> = &self.0;
        ToSql::<Array<Integer>, Pg>::to_sql(vec, out)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromSqlRow, AsExpression)]
#[diesel(sql_type = Array<Nullable<Integer>>)]
pub struct NullableIntArray(pub Vec<Option<i32>>);

impl FromSql<Array<Nullable<Integer>>, Pg> for NullableIntArray {
    fn from_sql(bytes: <Pg as backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let vec = Vec::<Option<i32>>::from_sql(bytes)?;
        Ok(NullableIntArray(vec))
    }
}

impl ToSql<Array<Nullable<Integer>>, Pg> for NullableIntArray {
    fn to_sql<'b>(&'b self, out: &mut serialize::Output<'b, '_, Pg>) -> serialize::Result {
        let vec = &self.0;
        ToSql::<Array<Nullable<Integer>>, Pg>::to_sql(vec, out)
    }
}
