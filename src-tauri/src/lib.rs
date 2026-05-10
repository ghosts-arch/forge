use ::tauri::Manager;
use serde::Serialize;
use sqlx::prelude::*;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::State;

struct AppData {
    pool: SqlitePool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Model {
    uuid: String,
    name: String,
    fields: serde_json::Value,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct Asset {
    uuid: String,
    model_id: String,
    name: String,
    fields: serde_json::Value,
}

#[tauri::command]
async fn get_models(state: State<'_, AppData>) -> Result<Vec<Model>, String> {
    let models = sqlx::query_as::<_, Model>("SELECT * FROM models")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(models.into())
}

#[tauri::command]
async fn get_assets(state: State<'_, AppData>) -> Result<Vec<Asset>, String> {
    let assets = sqlx::query_as::<_, Asset>("SELECT * FROM assets")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(assets.into())
}

#[tauri::command]
async fn create_model(
    state: State<'_, AppData>,
    name: String,
    fields: serde_json::Value,
) -> Result<Model, String> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let parsed_fields = serde_json::to_string(&fields).unwrap_or_else(|_| "[]".to_string());
    let result = sqlx::query_as::<_, Model>(
        "INSERT INTO models (uuid, name, fields) VALUES  (?1, ?2, ?3) RETURNING *",
    )
    .bind(uuid)
    .bind(name)
    .bind(parsed_fields)
    .fetch_one(&state.pool)
    .await;
    match result {
        Ok(model) => Ok(model),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
async fn create_asset(
    state: State<'_, AppData>,
    name: String,
    model_id: String,
    fields: serde_json::Value,
) -> Result<Asset, String> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let parsed_fields = serde_json::to_string(&fields).unwrap_or_else(|_| "[]".to_string());
    let result = sqlx::query_as::<_, Asset>(
        "INSERT INTO assets (uuid, model_id, name, fields) VALUES (?1, ?2, ?3, ?4) RETURNING *",
    )
    .bind(uuid)
    .bind(model_id)
    .bind(name)
    .bind(parsed_fields)
    .fetch_one(&state.pool)
    .await;
    match result {
        Ok(model) => Ok(model),
        Err(e) => Err(format!("{}", e)),
    }
}
async fn connect_database(path: PathBuf) -> SqlitePool {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.expect("");
    sqlx::migrate!().run(&pool).await.expect("");
    pool
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = app.path().app_data_dir().expect("");
            println!("{:?}", path);
            std::fs::create_dir_all(&path).expect("Impossible de créer un dossier dans roaming");
            let db_path = path.join("database.db");
            println!("{:?}", db_path);
            let pool = tauri::async_runtime::block_on(connect_database(db_path));
            app.manage(AppData { pool });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_models,
            create_model,
            create_asset,
            get_assets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
