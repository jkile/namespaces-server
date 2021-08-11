// use crate::models::{user::User, comment::Comment};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use std::time::SystemTime;

use crate::schema::threads;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub last_posted: SystemTime,
    namespace_name: String
}