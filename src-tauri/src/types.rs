use serde::Deserialize;
use serde::Serialize;
use sqlx::SqlitePool;

pub struct AppData {
    pub pool: SqlitePool,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Asset {
    pub uuid: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[sqlx(skip)]
    pub fields: Vec<AssetField>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AssetField {
    pub uuid: String,
    pub asset_id: String,
    pub name: String,
    pub kind: String,
    pub text_value: Option<String>,
    pub number_value: Option<f64>,
    pub date_value: Option<chrono::NaiveDate>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AssetFieldPayload {
    pub asset_id: String,
    pub name: String,
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct NewAssetField {
    pub uuid: String,
    pub asset_id: String,
    pub name: String,
    pub kind: String,
    pub text_value: Option<String>,
    pub number_value: Option<f64>,
    pub date_value: Option<chrono::NaiveDate>,
}
