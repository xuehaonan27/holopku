// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "GameType"))]
    pub struct GameType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "GoodsType"))]
    pub struct GoodsType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "LoginProvider"))]
    pub struct LoginProvider;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Place"))]
    pub struct Place;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "PostType"))]
    pub struct PostType;
}

diesel::table! {
    Comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        content -> Text,
        likes -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PostType;
    use super::sql_types::Place;
    use super::sql_types::GameType;
    use super::sql_types::GoodsType;

    Posts (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        user_id -> Int4,
        content -> Text,
        likes -> Int4,
        favorates -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        comments_id -> Array<Nullable<Int4>>,
        images -> Array<Nullable<Int4>>,
        post_type -> PostType,
        #[max_length = 255]
        contact -> Nullable<Varchar>,
        food_place -> Nullable<Place>,
        score -> Nullable<Int4>,
        people_all -> Nullable<Int4>,
        people_already -> Nullable<Int4>,
        game_type -> Nullable<GameType>,
        start_time -> Nullable<Timestamp>,
        #[max_length = 255]
        amuse_place -> Nullable<Varchar>,
        price -> Nullable<Int4>,
        goods_type -> Nullable<GoodsType>,
        sold -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoginProvider;

    Users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 100]
        email -> Nullable<Varchar>,
        login_provider -> LoginProvider,
        nickname -> Varchar,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(Comments -> Posts (post_id));
diesel::joinable!(Comments -> Users (user_id));
diesel::joinable!(Posts -> Users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Comments,
    Posts,
    Users,
);
