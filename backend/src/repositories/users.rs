use diesel::prelude::*;
use diesel::PgConnection;
use crate::models::users::{
    NewUser, 
    User
};
use crate::DbError;

pub struct UsersRepository;

impl UsersRepository {
    pub fn find_user_by_username(c: &mut PgConnection, user_name: &String) -> Result<Option<User>, DbError> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(user_name))
            .first::<User>(c)
            .optional()?;
        Ok(user)
    }

    pub fn create(c: &mut PgConnection, new_user: NewUser) -> Result<NewUser, DbError> {
        use crate::schema::users::dsl::*;
        let new_user_response = new_user.clone();
        diesel::insert_into(users)
            .values(&new_user)
            .execute(c)?;
        Ok(new_user_response)
    }
}