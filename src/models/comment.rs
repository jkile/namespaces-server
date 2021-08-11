use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};
use std::time::SystemTime;

use crate::schema::comments;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Comment {
    pub id: i32,
    pub author_id: i32,
    pub timestamp: SystemTime,
    pub contents: String,
    pub thread_position: i32,
    pub thread_id: i32
}