pub mod project_id;
pub mod project_meta;

use sqlx::{Pool, Sqlite};

pub struct Project {
    db_pool: Pool<Sqlite>,
}
