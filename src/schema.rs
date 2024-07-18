// @generated automatically by Diesel CLI.

diesel::table! {
    user_histories (username) {
        username -> Varchar,
        requests -> Array<Nullable<Jsonb>>,
    }
}

diesel::table! {
    users (username) {
        username -> Varchar,
        password_hashed -> Varchar,
    }
}

diesel::joinable!(user_histories -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    user_histories,
    users,
);
