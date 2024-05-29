use diesel::prelude::*;
use diesel::PgConnection;
use crate::models::users::{
    NewUser, 
    User
};
use crate::DbError;
use crate::schema::users::dsl::*;

pub struct UsersRepository;

impl UsersRepository {

    pub fn get_y1_y2(c: &mut PgConnection, user_name: &String) -> Result<(Vec<u8>, Vec<u8>), DbError> {
        match users
            .filter(username.eq(user_name))
            .first::<User>(c)
            .optional() {
                Ok(Some(found_user)) => Ok((found_user.y1, found_user.y2)),
                Ok(None) => Err("User not found".into()),
                Err(e) => Err(e.into()),
            }
    }

    pub fn _get_user_state(c: &mut PgConnection, user_name: &String) -> Result<String, DbError> {
        match users
            .filter(username.eq(user_name))
            .first::<User>(c)
            .optional() {
                Ok(Some(found_user)) => Ok(found_user.state),
                Ok(None) => Err("User not found".into()),
                Err(e) => Err(e.into()),
            }
    }

    pub fn update_user(c:&mut PgConnection, update_user: NewUser) -> Result<(), DbError> {
        let target_user = users.filter(username.eq(update_user.username));
        diesel::update(target_user)
            .set((
                state.eq("Active"),
                y1.eq(update_user.y1),
                y2.eq(update_user.y2)
            ))
            .execute(c)?;
        Ok(())
    }

    pub fn change_user_state(c: &mut PgConnection, user_name: &String, user_state: &str) -> Result<(), DbError> {
        let target_user = users.filter(username.eq(user_name));
        diesel::update(target_user)
            .set(state.eq(user_state))  
            .execute(c)?;
        Ok(())
    }

    pub fn find_user_by_username(c: &mut PgConnection, user_name: &String) -> Result<Option<User>, DbError> {
        let user = users
            .filter(username.eq(user_name))
            .first::<User>(c)
            .optional()?;
        Ok(user)
    }

    pub fn create(c: &mut PgConnection, new_user: NewUser) -> Result<NewUser, DbError> {
        let new_user_response = new_user.clone();
        diesel::insert_into(users)
            .values(&new_user)
            .execute(c)?;
        Ok(new_user_response)
    }
}