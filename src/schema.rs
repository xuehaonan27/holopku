// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "LoginProvider"))]
    pub struct LoginProvider;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoginProvider;

    User (id) {
        id -> Varchar,
        username -> Varchar,
        email -> Nullable<Varchar>,
        login_provider -> LoginProvider,
        nickname -> Varchar,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
