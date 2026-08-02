use std::path::PathBuf;

use error::Result;

pub mod error;

pub struct StormMateria {
    conn: sqlx::SqlitePool,
    internal_files: bool,
}

impl StormMateria {
    pub async fn new(target: ConnectionTarget, internal_files: bool) -> Result<Self> {
        let conn = sqlx::SqlitePool::connect(&target.to_url()).await?;
        sqlx::migrate!().run(&conn).await?;

        Ok(Self {
            conn,
            internal_files,
        })
    }
}

pub enum ConnectionTarget {
    File(PathBuf),
    Memory,
}

impl ConnectionTarget {
    fn to_url(&self) -> String {
        match self {
            Self::File(path) => format!("sqlite://{}?mode=rwc", path.to_string_lossy()),
            Self::Memory => "sqlite::memory:".into(),
        }
    }
}
