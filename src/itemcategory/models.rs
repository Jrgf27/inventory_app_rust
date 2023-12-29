use serde::{Deserialize, Serialize};
use sqlx::Sqlite;

#[derive(Serialize, Deserialize)]
pub struct ItemCategory {
    pub id: i64,
    pub name: String,
    pub description: String,
}

pub static MODEL_NAME: &str = stringify!(ItemCategory);

pub async fn db_insert_values(name: &str, description: &str) {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = format!(
        "INSERT INTO {} 
        (name, description) 
        VALUES (?, ?);",
        MODEL_NAME,
    );

    let result = sqlx::query(&db_query)
        .bind(name)
        .bind(description)
        .execute(&db)
        .await
        .unwrap();
    println!("Create user table result: {:?}", result);
}

pub async fn db_init() {
    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let db_query = format!(
        "CREATE TABLE IF NOT EXISTS {} 
        (id INTEGER PRIMARY KEY NOT NULL, 
        name VARCHAR(250) NOT NULL, 
        description VARCHAR(250) NOT NULL);",
        MODEL_NAME,
    );

    let result = sqlx::query(&db_query).execute(&db).await.unwrap();
    println!("Create user table result: {:?}", result);
}
