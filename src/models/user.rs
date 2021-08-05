use crate::models::{comment::Comment};
use serde::{Deserialize, Serialize};

type ImageUrl = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    // pub avatar: ImageUrl,
    // pub comments: Vec<Comment>
}