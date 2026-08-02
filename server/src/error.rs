#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database operation failed: {0}")]
    DbError(
        #[from]
        #[source]
        sqlx::Error,
    ),

    #[error("Database initilization failed: {0}")]
    DbMigrationError(
        #[from]
        #[source]
        sqlx::migrate::MigrateError,
    ),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;
