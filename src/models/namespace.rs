// use crate::models::{user::User, thread::Thread};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

use crate::schema::namespaces;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Namespace {
    pub name: String,
    pub owner_id: i32
}