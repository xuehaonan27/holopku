// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "LoginProvider"))]
    pub struct LoginProvider;
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
