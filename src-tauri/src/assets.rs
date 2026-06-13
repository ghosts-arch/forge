use std::collections::{HashMap, HashSet};

use crate::types::{self};

pub async fn create_asset(pool: &sqlx::SqlitePool, name: &str) -> Result<types::Asset, String> {
    if name.len() == 0 {
        return Err(format!("Le nom de l'asset est manquant"));
    };
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
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let mut asset = sqlx::query_as::<_, types::Asset>(
        "UPDATE assets SET name = ?1 WHERE uuid = ?2 RETURNING *",
    )
    .bind(name)
    .bind(&asset_uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    let currents_fields = sqlx::query_as::<_, types::AssetField>(
        "SELECT * FROM assets_fields WHERE assets_fields.asset_id = ?1",
    )
    .bind(&asset_uuid)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let hashed_current_fields = currents_fields
        .into_iter()
        .map(|field| field.uuid.clone())
        .collect::<HashSet<_>>();
    let mut new_fields: Vec<types::AssetFieldPayload> = Vec::new();
    let mut existing_fields: HashMap<String, types::AssetFieldPayload> = HashMap::new();
    for field in &fields_payload {
        match field.uuid.as_ref() {
            Some(x) => {
                existing_fields.insert(x.clone(), field.clone());
            }
            None => new_fields.push(field.clone()),
        }
    }
    let hashed_new_fields_payload = new_fields
        .clone()
        .into_iter()
        .map(types::NewAssetField::try_from)
        .collect::<Result<Vec<types::NewAssetField>, _>>()?;
    let hashed_fields_payload = existing_fields
        .clone()
        .into_iter()
        .map(|field| (field.0, types::NewAssetField::try_from(field.1)))
        .collect::<HashMap<_, _>>();
    let hashed_fields_uuid: HashSet<String> = existing_fields.keys().cloned().collect();
    let deleted_fields_uuid = hashed_current_fields.difference(&hashed_fields_uuid);
    for field in hashed_new_fields_payload {
        sqlx::query("INSERT INTO assets_fields (uuid, asset_id, name, kind, text_value, number_value, date_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)").bind(field.uuid).bind(&asset_uuid).bind(field.name).bind(field.kind).bind(field.text_value).bind(field.number_value).bind(field.date_value).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    for (_, field) in hashed_fields_payload {
        match field {
            Ok(f) => {
                sqlx::query("UPDATE assets_fields SET name = ?1, kind = ?2, text_value = ?3, number_value = ?4, date_value = ?5 WHERE uuid = ?6").bind(f.name).bind(f.kind).bind(f.text_value).bind(f.number_value).bind(f.date_value).bind(f.uuid).execute(&mut *tx).await.map_err(|e| e.to_string())?;
            }
            Err(e) => return Err(format!("{}", e)),
        }
    }
    for field_uuid in deleted_fields_uuid {
        sqlx::query("DELETE FROM assets_fields where uuid = ?1")
            .bind(&field_uuid)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
    }
    let fields = sqlx::query_as::<_, types::AssetField>(
        "SELECT *  FROM assets_fields WHERE assets_fields.asset_id = ?1",
    )
    .bind(&asset_uuid)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;
    asset.fields = fields;
    tx.commit().await.map_err(|e| e.to_string())?;
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

pub async fn search_assets(
    pool: &sqlx::SqlitePool,
    query: String,
) -> Result<Vec<types::Asset>, String> {
    let prepared_query = format!("%{}%", query);
    let assets = sqlx::query_as::<_, types::Asset>(
        "SELECT DISTINCT assets.* FROM assets LEFT JOIN assets_fields ON assets.uuid = assets_fields.asset_id WHERE assets.name LIKE ?1 OR assets_fields.name LIKE ?2 OR assets_fields.text_value LIKE ?3",
    )
    .bind(&prepared_query)
    .bind(&prepared_query)
    .bind(&prepared_query)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(assets)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_create_asset_with_empty_name() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .expect("");
        sqlx::migrate!().run(&pool).await.expect("");
        let result = create_asset(&pool, "").await.unwrap_err();
        assert_eq!(result, "Le nom de l'asset est manquant")
    }
}
