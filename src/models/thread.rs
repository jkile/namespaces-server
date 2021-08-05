use crate::models::{user::User, comment::Comment};
use serde::{Deserialize, Serialize};

pub type Date = String;

#[derive(Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub author: User,
    pub last_posted: Date,
    pub comments: Vec<Comment>
}