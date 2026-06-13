use crate::types;

pub async fn create_relation(
    pool: &sqlx::SqlitePool,
    source_asset_uuid: String,
    target_asset_uuid: String,
    description: String,
) -> Result<types::Relation, String> {
    let uuid = uuid::Uuid::new_v4().to_string();
    let relation = sqlx::query_as::<_, types::Relation>("INSERT INTO relations (uuid, description , source_asset_uuid, target_asset_uuid ) VALUES (?1, ?2, ?3, ?4) RETURNING * ").bind(uuid).bind(description).bind(source_asset_uuid).bind(target_asset_uuid).fetch_one(pool).await.map_err(|e| e.to_string())?;
    Ok(relation)
}

pub async fn get_relations(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<types::AssetInformations>, String> {
    let relations = sqlx::query_as::<_, types::AssetInformations>("SELECT name, uuid FROM assets")
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(relations)
}

pub async fn get_relations_for(
    pool: &sqlx::SqlitePool,
    source: String,
) -> Result<Vec<types::Relation>, String> {
    let relations = sqlx::query_as::<_, types::Relation>(
        "SELECT relations.uuid, relations.description, relations.source_asset_uuid , relations.target_asset_uuid, assets.name FROM relations INNER JOIN assets ON assets.uuid = relations.target_asset_uuid WHERE relations.source_asset_uuid = ?1",
    )
    .bind(source)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(relations)
}

pub async fn get_relations_from(
    pool: &sqlx::SqlitePool,
    source: String,
) -> Result<Vec<types::Relation>, String> {
    let relations = sqlx::query_as::<_, types::Relation>(
        "SELECT relations.uuid, relations.description, relations.source_asset_uuid , relations.target_asset_uuid, assets.name FROM relations INNER JOIN assets ON assets.uuid = relations.source_asset_uuid WHERE relations.target_asset_uuid = ?1",
    )
    .bind(source)
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(relations)
}
