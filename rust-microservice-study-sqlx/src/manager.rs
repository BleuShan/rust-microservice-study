use sqlx::PgPool;

use crate::prelude::*;

#[derive(Debug, DerefMut, Deref)]
/// Manage database connection.
pub struct DatabaseManager {
    #[deref]
    pool: PgPool,
}
