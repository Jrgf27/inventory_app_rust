use crate::itemcategory::forms::ItemCategoryForm;
use crate::orm::utils::{GenericModel, ModelMethods};
use serde::{Deserialize, Serialize};
use sqlx::{Row, Sqlite};

#[derive(Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: GenericModel,
    pub name: GenericModel,
    pub description: GenericModel,
}

impl ItemCategory {
    pub fn new_default() -> Self {
        ItemCategory {
            id: GenericModel::IntegerField(0),
            name: GenericModel::CharField(String::from("")),
            description: GenericModel::CharField(String::from("")),
            // Initialize additional fields with default values
            // additional_field: GenericModel::...,
        }
    }
}

impl ModelMethods for ItemCategory {}

pub async fn db_insert_values(payload: &ItemCategoryForm) -> Result<i64, sqlx::Error> {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let default_itemcat = ItemCategory::new_default();

    let db_query = default_itemcat.insert_values(payload);

    sqlx::query(&db_query).execute(&db).await.unwrap();

    let row = sqlx::query("SELECT id FROM ItemCategory ORDER BY id DESC LIMIT 1")
        .fetch_one(&db)
        .await
        .unwrap();

    let id: i64 = row.try_get("id")?;

    Ok(id)
}

pub async fn db_init() {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let default_itemcat = ItemCategory::new_default();

    let db_query = default_itemcat.db_init();

    sqlx::query(&db_query).execute(&db).await.unwrap();
}
