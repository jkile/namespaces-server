use crate::models::{user::User, thread::Thread};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Namespace {
    pub id: String,
    pub owner: User,
    pub members: Vec<User>,
    pub threads: Vec<Thread>
}