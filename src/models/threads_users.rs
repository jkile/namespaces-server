use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::threads_users;

#[derive(Deserialize, Serialize, Queryable, Insertable)]
pub struct threads_users {
    pub author_id: i32,
    pub thread_id: i32
}