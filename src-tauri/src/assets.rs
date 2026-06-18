use crate::types::{self};
use std::collections::{HashMap, HashSet};

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
            .map_err(|_| format!("L'uuid : {} n'existe pas", &uuid))?;
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
    if name.is_empty() {
        return Err(format!("Le nom de l'asset est manquant"));
    };
    let (existing_fields, new_fields): (
        Vec<types::AssetFieldPayload>,
        Vec<types::AssetFieldPayload>,
    ) = fields_payload.into_iter().partition(|f| f.uuid.is_some());
    let existing_fields_map = existing_fields
        .into_iter()
        .map(|mut f| (f.uuid.take().unwrap(), f))
        .collect::<HashMap<String, types::AssetFieldPayload>>();
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
    let mut asset = sqlx::query_as::<_, types::Asset>(
        "UPDATE assets SET name = ?1 WHERE uuid = ?2 RETURNING *",
    )
    .bind(name)
    .bind(&asset_uuid)
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| "L'asset n'as pas été trouvé dans la db")?;
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
    let deleted_fields = hashed_current_fields
        .into_iter()
        .filter(|k| !existing_fields_map.contains_key(k))
        .collect::<Vec<_>>();
    for field in new_fields {
        let new_asset_field = types::NewAssetField::try_from(field.clone()).unwrap();
        sqlx::query("INSERT INTO assets_fields (uuid, asset_id, name, kind, text_value, number_value, date_value) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)").bind(new_asset_field.uuid).bind(new_asset_field.asset_id).bind(new_asset_field.name).bind(new_asset_field.kind).bind(new_asset_field.text_value).bind(new_asset_field.number_value).bind(new_asset_field.date_value).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    for (uuid, mut field) in existing_fields_map {
        field.uuid = Some(uuid);
        let new_asset_field = types::NewAssetField::try_from(field).unwrap();
        sqlx::query("UPDATE assets_fields SET name = ?1, kind = ?2, text_value = ?3, number_value = ?4, date_value = ?5 WHERE uuid = ?6").bind(new_asset_field.name).bind(new_asset_field.kind).bind(new_asset_field.text_value).bind(new_asset_field.number_value).bind(new_asset_field.date_value).bind(&new_asset_field.uuid).execute(&mut *tx).await.map_err(|e| e.to_string())?;
    }
    for field_uuid in deleted_fields {
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
        .map_err(|_| "No asset found for this uuid")?;
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
    let prepared_query = format!("%{}%", query.replace("%", "\\%").replace("_", "\\_"));
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
    async fn test_create_asset_with_empty_name(pool: sqlx::SqlitePool) {
        let result = create_asset(&pool, "").await.unwrap_err();
        assert_eq!(result, "Le nom de l'asset est manquant")
    }

    #[sqlx::test]
    async fn test_create_asset_with_name(pool: sqlx::SqlitePool) {
        let result = create_asset(&pool, "test asset").await.unwrap();
        assert_eq!(result.name, "test asset")
    }

    #[sqlx::test]
    async fn test_get_asset_with_non_existing_uuid(pool: sqlx::SqlitePool) {
        let result = get_asset(&pool, "non-existing-uuid").await.unwrap_err();
        assert_eq!(result, format!("L'uuid : non-existing-uuid n'existe pas"))
    }

    #[sqlx::test]
    async fn test_get_asset_with_existing_uuid(pool: sqlx::SqlitePool) {
        let created_asset = create_asset(&pool, "existing-uuid").await.unwrap();
        let result = get_asset(&pool, &created_asset.uuid).await.unwrap();
        assert_eq!(result, created_asset)
    }

    #[sqlx::test]
    async fn test_update_asset_with_empty_name(pool: sqlx::SqlitePool) {
        let created_asset = create_asset(&pool, "creating asset").await.unwrap();
        let fake_fields_payload: Vec<types::AssetFieldPayload> = Vec::new();
        let updated_asset = update_asset(&pool, "", created_asset.uuid, fake_fields_payload)
            .await
            .unwrap_err();
        assert_eq!(updated_asset, "Le nom de l'asset est manquant")
    }

    #[sqlx::test]
    async fn test_update_asset_name(pool: sqlx::SqlitePool) {
        let created_asset = create_asset(&pool, "creating asset").await.unwrap();
        let fake_fields_payload: Vec<types::AssetFieldPayload> = Vec::new();
        let updated_asset = update_asset(
            &pool,
            "updating asset",
            created_asset.uuid,
            fake_fields_payload,
        )
        .await
        .unwrap();
        assert_eq!(updated_asset.name, "updating asset")
    }

    #[sqlx::test]
    async fn test_update_non_existing_asset(pool: sqlx::SqlitePool) {
        let fake_fields_payload: Vec<types::AssetFieldPayload> = Vec::new();
        let updated_asset = update_asset(
            &pool,
            "updating asset",
            String::from("non existing id"),
            fake_fields_payload,
        )
        .await
        .unwrap_err();
        assert_eq!(updated_asset, "L'asset n'as pas été trouvé dans la db")
    }

    #[sqlx::test]
    async fn test_insert_asset_field(pool: sqlx::SqlitePool) {
        let asset = create_asset(&pool, "creating asset").await.unwrap();
        let mut inserted_fields = Vec::new();
        inserted_fields.push(types::AssetFieldPayload {
            asset_id: asset.uuid.clone(),
            name: String::from("testing field"),
            uuid: None,
            kind: types::Kind::Text,
            value: String::from("placeholder"),
        });
        let result = update_asset(&pool, "creating asset", asset.uuid, inserted_fields)
            .await
            .unwrap();
        assert_eq!(
            result.fields[0].text_value,
            Some(String::from("placeholder"))
        );
        assert_eq!(result.fields[0].number_value, None);
        assert_eq!(result.fields[0].date_value, None);
        assert_eq!(result.fields[0].kind, types::Kind::Text);
        assert_eq!(result.fields[0].name, String::from("testing field"));
        assert!(uuid::Uuid::try_parse(&result.fields[0].uuid).is_ok());
    }

    #[sqlx::test]
    async fn test_delete_non_existing_asset(pool: sqlx::SqlitePool) {
        let asset = delete_asset(&pool, "non existing uuid").await.unwrap();
        assert_eq!(asset, false)
    }

    #[sqlx::test]
    async fn test_delete_asset(pool: sqlx::SqlitePool) {
        let asset = create_asset(&pool, "non existing uuid").await.unwrap();
        let result = delete_asset(&pool, &asset.uuid).await.unwrap();
        assert_eq!(result, true);
        let non_existing_asset = get_asset(&pool, &asset.uuid).await.unwrap_err();
        assert_eq!(
            non_existing_asset,
            format!("L'uuid : {} n'existe pas", &asset.uuid)
        )
    }
}
