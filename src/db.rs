use sqlx::{migrate::MigrateDatabase, Result, Sqlite, SqlitePool};

pub async fn create_temp_db() -> Result<SqlitePool> {
  open_some_db(None).await
}

pub async fn open_db(path: &str) -> Result<SqlitePool> {
  open_some_db(Some(path)).await
}

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
