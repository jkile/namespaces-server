use crate::models::{user::User, thread::Date};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: User,
    pub timestamp: Date,
    pub contents: String
}