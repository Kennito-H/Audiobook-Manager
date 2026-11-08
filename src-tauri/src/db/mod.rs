use std::path::Path;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};

use crate::error::AppResult;

/// Opens (creating if needed) the library database at `path` and applies migrations.
pub async fn connect(path: &Path) -> AppResult<SqlitePool> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn migrations_apply_on_fresh_db() {
        let dir = std::env::temp_dir().join(format!("abm-test-{}", std::process::id()));
        let path = dir.join("library.db");
        let _ = std::fs::remove_file(&path);
        let pool = connect(&path).await.expect("connect");
        let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings")
            .fetch_one(&pool)
            .await
            .expect("settings table exists");
        assert_eq!(n, 0);
        pool.close().await;
        let _ = std::fs::remove_dir_all(dir);
    }
}
