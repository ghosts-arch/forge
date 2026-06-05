use std::collections::HashMap;

use crate::types::{self};

pub async fn create_asset(pool: &sqlx::SqlitePool, name: &str) -> Result<types::Asset, String> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query_as::<_, types::Asset>(
        "INSERT INTO assets (uuid, name) VALUES (?1, ?2) RETURNING *",
    )
    .bind(uuid)
    .bind(name)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(result)
}

pub async fn get_asset(pool: &sqlx::SqlitePool, uuid: &str) -> Result<types::Asset, String> {
    let mut asset =
        sqlx::query_as::<_, types::Asset>("SELECT * FROM assets WHERE assets.uuid = ?1")
            .bind(&uuid)
            .fetch_one(pool)
            .await
            .map_err(|e| e.to_string())?;
    let fields = sqlx::query_as::<_, types::AssetField>(
        "SELECT *  FROM assets_fields WHERE assets_fields.asset_id = ?1",
    )
    .bind(uuid)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    asset.fields = fields;
    Ok(asset)
}

pub async fn get_assets(pool: &sqlx::SqlitePool) -> Result<Vec<types::Asset>, String> {
    let assets = sqlx::query_as::<_, types::Asset>("SELECT * FROM assets")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
    let fields = sqlx::query_as::<_, types::AssetField>("SELECT * FROM assets_fields")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
    let mut mapped_assets = HashMap::new();
    for field in fields {
        mapped_assets
            .entry(field.asset_id.clone())
            .or_insert(Vec::new())
            .push(field);
    }
    let updated_assets: Vec<types::Asset> = assets
        .into_iter()
        .map(|mut asset| {
            let f = mapped_assets.remove(&asset.uuid).unwrap_or_default();
            asset.fields = f;
            asset
        })
        .collect();
    Ok(updated_assets)
}

pub async fn update_asset(
    pool: &sqlx::SqlitePool,
    name: &str,
    asset_uuid: String,
    fields_payload: Vec<types::AssetFieldPayload>,
) -> Result<types::Asset, String> {
    let fields: Vec<types::NewAssetField> = fields_payload
        .into_iter()
        .map(|payload| {
            let mut text_value = None;
            let mut number_value = None;
            let mut date_value = None;
            match payload.kind.as_str() {
                "text" => text_value = Some(payload.value),
                "number" => number_value = payload.value.parse::<f64>().ok(),
                "date" => {
                    date_value = chrono::NaiveDate::parse_from_str(&payload.value, "%Y-%m-%d").ok()
                }
                _ => {}
            }
            types::NewAssetField {
                asset_id: asset_uuid.clone(),
                uuid: uuid::Uuid::new_v4().to_string(),
                name: payload.name,
                kind: payload.kind,
                text_value,
                number_value,
                date_value,
            }
        })
        .collect();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let mut asset = sqlx::query_as::<_, types::Asset>(
        "UPDATE assets SET name = ?1 WHERE uuid = ?2 RETURNING *",
    )
    .bind(name)
    .bind(&asset_uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM assets_fields where asset_id = ?1")
        .bind(&asset_uuid)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    for field in fields {
        sqlx::query("INSERT INTO assets_fields (uuid, asset_id, name, kind, text_value, number_value, date_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)").bind(field.uuid).bind(&asset_uuid).bind(field.name).bind(field.kind).bind(field.text_value).bind(field.number_value).bind(field.date_value).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    tx.commit().await.map_err(|e| e.to_string())?;
    let fields = sqlx::query_as::<_, types::AssetField>(
        "SELECT *  FROM assets_fields WHERE assets_fields.asset_id = ?1",
    )
    .bind(&asset_uuid)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    asset.fields = fields;
    Ok(asset)
}

pub async fn delete_asset(pool: &sqlx::SqlitePool, uuid: &str) -> Result<bool, String> {
    let result = sqlx::query("DELETE FROM assets WHERE uuid = ?1")
        .bind(uuid)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    if result.rows_affected() > 0 {
        Ok(true)
    } else {
        Ok(false)
    }
}
