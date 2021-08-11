use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Deserialize, Serialize, Queryable, Insertable)]
pub struct namespaces_users {
    pub user_id: i32,
    pub namespaces_name: String
}