use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppResult;

#[derive(Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub db_ok: bool,
}

/// Basic health check used by the UI on startup.
#[tauri::command]
#[specta::specta]
pub async fn app_info(app: tauri::AppHandle, pool: State<'_, SqlitePool>) -> AppResult<AppInfo> {
    let db_ok = sqlx::query("SELECT 1").execute(pool.inner()).await.is_ok();
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        db_ok,
    })
}
