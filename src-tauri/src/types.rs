use serde::Deserialize;
use serde::Serialize;
use sqlx::SqlitePool;

pub struct AppData {
    pub pool: SqlitePool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Text,
    Number,
    Date,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Relation {
    pub uuid: String,
    pub description: String,
    pub source_asset_uuid: String,
    pub target_asset_uuid: String,
    pub name: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AssetInformations {
    pub uuid: String,
    pub name: String,
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
    pub kind: Kind,
    pub text_value: Option<String>,
    pub number_value: Option<f64>,
    pub date_value: Option<chrono::NaiveDate>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct AssetFieldPayload {
    pub uuid: Option<String>,
    pub asset_id: String,
    pub name: String,
    pub kind: Kind,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct NewAssetField {
    pub uuid: String,
    pub asset_id: String,
    pub name: String,
    pub kind: Kind,
    pub text_value: Option<String>,
    pub number_value: Option<f64>,
    pub date_value: Option<chrono::NaiveDate>,
}

impl TryFrom<AssetFieldPayload> for NewAssetField {
    type Error = String;
    fn try_from(value: AssetFieldPayload) -> Result<Self, Self::Error> {
        let mut text_value = None;
        let mut number_value = None;
        let mut date_value = None;
        match value.kind {
            Kind::Text => text_value = Some(value.value),
            Kind::Number => {
                number_value = Some(value.value.parse::<f64>().map_err(|e| format!("{}", e))?)
            }
            Kind::Date => {
                date_value = Some(
                    chrono::NaiveDate::parse_from_str(&value.value, "%Y-%m-%d")
                        .map_err(|e| format!("{}", e))?,
                )
            }
            _ => return Err(format!("")),
        }

        let uuid: String;
        match value.uuid {
            Some(x) => uuid = x,
            None => uuid = uuid::Uuid::new_v4().to_string(),
        }

        Ok(NewAssetField {
            asset_id: value.asset_id,
            uuid: uuid,
            name: value.name,
            kind: value.kind,
            text_value,
            number_value,
            date_value,
        })
    }
}
