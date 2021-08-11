use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use diesel::{Queryable, Insertable};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    created_on: SystemTime,
    last_login: Option<SystemTime>
    // pub avatar: ImageUrl,
    // pub comments: Vec<Comment>
}