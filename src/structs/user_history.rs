use diesel::{ExpressionMethods, insert_into, OptionalExtension, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use diesel::{self, prelude::*};
use serde_json::Value;
use diesel::pg::sql_types::Array;
use crate::user_history::diesel::pg::sql_types::Jsonb;

#[derive(QueryableByName, AsChangeset, Insertable)]
#[diesel(table_name = crate::schema::user_histories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserHistory {
    pub username: String,
    #[sql_type = "Array<Jsonb>"]
    pub requests: Vec<Value>
}



