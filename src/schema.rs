// @generated automatically by Diesel CLI.

diesel::table! {
    alembic_version (version_num) {
        #[max_length = 32]
        version_num -> Varchar,
    }
}

diesel::table! {
    subscriptions (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Nullable<Varchar>,
        hashed_password -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(subscriptions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    alembic_version,
    subscriptions,
    users,
);
