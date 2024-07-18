use diesel::{ExpressionMethods, insert_into, OptionalExtension, PgConnection, RunQueryDsl};
use crate::schema::users::*;
use diesel::{self, prelude::*};
use serde::Serialize;
use serde_json::json;
use crate::error_types::SignUpError;
use crate::schema::user_histories::dsl::user_histories;
use crate::schema::users::dsl::users;
use crate::user_history::UserHistory;

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_hashed: String
}
impl User {
    pub fn find_by_username(conn: &mut PgConnection, username_s: &str) -> Result<Option<User>, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let user = users.filter(username.eq(username_s))
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }
    pub fn create_new(mut self, conn: &mut PgConnection) -> Result<Self, SignUpError> {
        use crate::schema::user_histories::dsl::*;

        match User::find_by_username(conn, &self.username) {
            Ok(Some(_)) => {
                return Err(SignUpError::UsernameTakenError)
            }
            Ok(None) => {
                // User does not exist, create a new user
                insert_into(users).values(&self).execute(conn)?;

                // Create user history
                let new_user_his = UserHistory {
                    username: self.username.clone(),
                    requests: vec![], // Assuming default is an empty JSON array
                };
                insert_into(user_histories).values(new_user_his).execute(conn)?;
            }
            Err(e) => {
                return Err(SignUpError::from(e));
            }
        }

        Ok(self)

    }
    pub async fn verify_password(&self, input_password: String) -> bool{
        if bcrypt::verify(input_password, &self.password_hashed).expect("Verify") {
            true
        } else {
            false
        }
    }
}