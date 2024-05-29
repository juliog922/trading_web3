use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    #[serde(skip_deserializing)]
    pub state: String, 
    pub y1: Vec<u8>,
    pub y2: Vec<u8>
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub y1: Vec<u8>,
    pub y2: Vec<u8>
}