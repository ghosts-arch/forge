use ::tauri::Manager;
use serde::Deserialize;
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

#[derive(Debug, Serialize, sqlx::FromRow)]
struct AssetWithModel {
    uuid: String,
    name: String,
    fields: serde_json::Value,
    model_uuid: String,
    model_name: String,
    model_fields: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct ModelField {
    name: String,
    kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AssetField {
    name: String,
    value: String,
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
async fn get_model(state: State<'_, AppData>, uuid: String) -> Result<Model, String> {
    let result = sqlx::query_as::<_, Model>("SELECT * FROM models WHERE uuid = ?1")
        .bind(uuid)
        .fetch_one(&state.pool)
        .await;

    match result {
        Ok(model) => Ok(model),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
async fn get_asset(state: State<'_, AppData>, uuid: String) -> Result<Asset, String> {
    let result = sqlx::query_as::<_, Asset>("SELECT * FROM assets WHERE uuid = ?1")
        .bind(uuid)
        .fetch_one(&state.pool)
        .await;
    match result {
        Ok(asset) => Ok(asset),
        Err(e) => Err(format!("{}", e)),
    }
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
    if name.is_empty() {
        return Err(format!("Name is empty"));
    };
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
    if name.is_empty() {
        return Err(format!("Name is empty"));
    };
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

#[tauri::command]
async fn add_model_field(
    state: State<'_, AppData>,
    field: ModelField,
    model_uuid: String,
) -> Result<Model, String> {
    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;
    let mut current_model = sqlx::query_as::<_, Model>("SELECT * FROM models WHERE uuid = ?1")
        .bind(&model_uuid)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    let new_field = serde_json::to_value(field).map_err(|e| e.to_string())?;
    if let Some(field_array) = current_model.fields.as_array_mut() {
        field_array.push(new_field);
    } else {
        return Err("error while parsing field".into());
    }
    let updated_model =
        sqlx::query_as::<_, Model>("UPDATE models SET fields = ?1 WHERE uuid = ?2 RETURNING *")
            .bind(current_model.fields)
            .bind(&model_uuid)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(updated_model)
}

#[tauri::command]
async fn update_asset_fields(
    state: State<'_, AppData>,
    new_fields: AssetField,
    uuid: String,
) -> Result<AssetWithModel, String> {
    let mut tx = state.pool.begin().await.map_err(|e| e.to_string())?;
    let mut current_model =
        sqlx::query_as::<_, AssetWithModel>("SELECT * FROM assets WHERE uuid = ?1")
            .bind(&uuid)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    let new_field = serde_json::to_value(new_fields).map_err(|e| e.to_string())?;
    if let Some(field_array) = current_model.fields.as_array_mut() {
        field_array.push(new_field);
    } else {
        return Err("error while parsing field".into());
    }
    let updated_model = sqlx::query_as::<_, AssetWithModel>(
        "UPDATE assets SET fields = ?1 WHERE uuid = ?2 RETURNING *",
    )
    .bind(current_model.fields)
    .bind(&uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(updated_model)
}

#[tauri::command]
async fn get_asset_with_model(
    state: State<'_, AppData>,
    uuid: String,
) -> Result<AssetWithModel, String> {
    let asset = sqlx::query_as::<_, AssetWithModel>(
        "SELECT assets.* , models.uuid AS model_uuid, models.name  AS model_name, models.fields AS model_fields  FROM assets LEFT JOIN models ON assets.model_id = models.uuid WHERE assets.uuid = ?1",
    )
    .bind(uuid)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(asset)
}

#[tauri::command]
async fn delete_model(state: State<'_, AppData>, uuid: String) -> Result<bool, String> {
    let result = sqlx::query("DELETE FROM models WHERE uuid = ?1")
        .bind(uuid)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    if result.rows_affected() > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn delete_asset(state: State<'_, AppData>, uuid: String) -> Result<bool, String> {
    let result = sqlx::query("DELETE FROM assets WHERE uuid = ?1")
        .bind(uuid)
        .execute(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    if result.rows_affected() > 0 {
        Ok(true)
    } else {
        Ok(false)
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
            std::fs::create_dir_all(&path).expect("Impossible de créer un dossier dans roaming");
            let db_path = path.join("database.db");
            let pool = tauri::async_runtime::block_on(connect_database(db_path));
            app.manage(AppData { pool });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_models,
            create_model,
            create_asset,
            get_assets,
            get_asset,
            get_asset_with_model,
            get_model,
            add_model_field,
            update_asset_fields,
            delete_model,
            delete_asset
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
