use crate::itemcategory::forms::ItemCategoryForm;
use crate::orm::utils::{GenericModel, ModelMethods};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, Sqlite};

#[derive(Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: GenericModel,
    pub name: GenericModel,
    pub description: GenericModel,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct ItemCategoryData {
    pub id: i64,
    pub name: String,
    pub description: String,
}

impl ItemCategory {
    pub fn new_default() -> Self {
        ItemCategory {
            id: GenericModel::IntegerField(0),
            name: GenericModel::CharField(String::from("")),
            description: GenericModel::CharField(String::from("")),
        }
    }
}

impl ModelMethods for ItemCategory {}

pub async fn db_init() {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = ItemCategory::new_default().db_init();

    sqlx::query(&db_query).execute(&db).await.unwrap();
}

pub async fn db_insert_values(payload: &ItemCategoryForm) -> Result<i64, sqlx::Error> {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = ItemCategory::new_default().insert_values(payload);

    sqlx::query(&db_query).execute(&db).await.unwrap();

    let db_query = ItemCategory::new_default().return_last_id();

    let row = sqlx::query(&db_query).fetch_one(&db).await.unwrap();

    let id: i64 = row.try_get("id")?;

    Ok(id)
}

pub async fn db_retrieve_all() -> Vec<ItemCategoryData> {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = ItemCategory::new_default().db_retrieve_all();

    sqlx::query_as::<_, ItemCategoryData>(&db_query)
        .fetch_all(&db)
        .await
        .unwrap()
}

pub async fn db_retrieve_by_id(id: i64) -> ItemCategoryData {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = ItemCategory::new_default().db_retrieve_by_id(id);

    sqlx::query_as::<_, ItemCategoryData>(&db_query)
        .fetch_one(&db)
        .await
        .unwrap()
}

pub async fn db_retrieve_by_field(field: &str, value: &str) -> ItemCategoryData {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = ItemCategory::new_default().db_retrieve_by_field(field, value);

    sqlx::query_as::<_, ItemCategoryData>(&db_query)
        .fetch_one(&db)
        .await
        .unwrap()
}
