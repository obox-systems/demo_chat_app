use sqlx::{migrate::MigrateDatabase, Result, Sqlite, SqlitePool};

/// Creates in memory DB.
pub async fn create_temp_db() -> Result<SqlitePool> {
  open_some_db(None).await
}

/// Opens or creates new SQLite DB from path.
pub async fn open_db(path: &str) -> Result<SqlitePool> {
  open_some_db(Some(path)).await
}

/// Opens or creates new SQLite DB from path if it's provided or creates in memory DB otherwise.
pub async fn open_some_db(path: Option<&str>) -> Result<SqlitePool> {
  let db = if let Some(path) = path {
    if !std::path::Path::new(path).exists() {
      std::fs::DirBuilder::new().create(path)?;
    }
    let sqlite = format!("{}/chat", path);
    Sqlite::create_database(&sqlite).await?;
    SqlitePool::connect(&sqlite).await?
  } else {
    SqlitePool::connect(":memory:").await?
  };
  sqlx::migrate!("./migrations").run(&db).await?;
  Ok(db)
}
