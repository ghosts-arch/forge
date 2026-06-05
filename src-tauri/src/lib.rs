use ::tauri::Manager;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::State;

mod assets;
mod types;

#[tauri::command]
async fn get_asset(state: State<'_, types::AppData>, uuid: String) -> Result<types::Asset, String> {
    assets::get_asset(&state.pool, &uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_assets(state: State<'_, types::AppData>) -> Result<Vec<types::Asset>, String> {
    assets::get_assets(&state.pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_asset(
    state: State<'_, types::AppData>,
    name: String,
) -> Result<types::Asset, String> {
    assets::create_asset(&state.pool, &name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_asset(
    state: State<'_, types::AppData>,
    uuid: String,
    name: String,
    fields: Vec<types::AssetFieldPayload>,
) -> Result<types::Asset, String> {
    assets::update_asset(&state.pool, &name, uuid, fields)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_asset(state: State<'_, types::AppData>, uuid: String) -> Result<bool, String> {
    assets::delete_asset(&state.pool, &uuid)
        .await
        .map_err(|e| e.to_string())
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
            std::fs::create_dir_all(&path).expect("Impossible de créer un dossier dans roaming");
            let db_path = path.join("database.db");
            let pool = tauri::async_runtime::block_on(connect_database(db_path));
            app.manage(types::AppData { pool });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_asset,
            get_assets,
            get_asset,
            update_asset,
            delete_asset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
