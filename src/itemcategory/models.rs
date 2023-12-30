use crate::itemcategory::forms::ItemCategoryForm;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Row, Sqlite};

#[derive(Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: GenericModel,
    pub name: GenericModel,
    pub description: GenericModel,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GenericModel {
    IntegerField(i64),
    CharField(String),
}

pub static MODEL_NAME: &str = stringify!(ItemCategory);

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

    pub fn db_init(&self) -> String {
        let json_value: Value = serde_json::to_value(self).expect("Failed to serialize struct");

        let mut res: String = "CREATE TABLE IF NOT EXISTS ".to_string()
            + MODEL_NAME
            + " (id INTEGER PRIMARY KEY NOT NULL,";

        if let Value::Object(fields) = json_value {
            for (field_name, field_value) in fields {
                // Match the field value with GenericModel type
                match serde_json::from_value::<GenericModel>(field_value) {
                    Ok(generic_model) => match generic_model {
                        GenericModel::IntegerField(_value) => {
                            if field_name == "id" {
                                continue;
                            } else {
                                res += &format!("{} INTEGER NOT NULL,", field_name);
                            }
                        }
                        GenericModel::CharField(_value) => {
                            res += &format!("{} VARCHAR(250) NOT NULL,", field_name);
                        }
                    },
                    Err(_) => println!("Unknown field type"),
                }
            }
        }
        let mut chars = res.chars();
        chars.next_back();
        res = chars.as_str().to_string().clone();

        res += ");";

        res
    }

    pub fn insert_values(&self, payload: &ItemCategoryForm) -> String {
        let json_value: Value = serde_json::to_value(self).expect("Failed to serialize struct");

        let json_payload: Value =
            serde_json::to_value(payload).expect("Failed to serialize struct");

        let mut res: String = "INSERT INTO ".to_string() + MODEL_NAME + " (";
        let mut values_to_insert: String = "VALUES (".to_string();

        if let Value::Object(fields) = json_value {
            for (field_name, _field_value) in fields {
                if let Value::Object(fields_payload) = &json_payload {
                    for (field_name_payload, field_value_payload) in fields_payload {
                        if field_name == "id" {
                            continue;
                        } else if field_name_payload == &field_name {
                            res += &format!(" {},", &field_name);

                            values_to_insert += &format!(" {},", &field_value_payload);
                        }
                    }
                }
            }
        }
        let mut chars = res.chars();
        chars.next_back();
        res = chars.as_str().to_string().clone();

        let mut chars = values_to_insert.chars();
        chars.next_back();
        values_to_insert = chars.as_str().to_string().clone();

        res += ")";
        res += &values_to_insert;
        res += ");";

        res
    }
}
