use axum::{
    routing::{get, post},
    Router,
};
use axum_csrf::CsrfConfig;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

mod itemcategory;
mod orm;

pub const DB_URL: &str = "sqlite://sqlite.sqlite3";

#[tokio::main]
async fn main() {
    let config = CsrfConfig::default();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(itemcategory::views::root))
        .route("/htmx", post(itemcategory::views::htmx))
        .with_state(config);

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    itemcategory::models::db_init().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn db_connection() -> sqlx::Pool<Sqlite> {
    match SqlitePool::connect(DB_URL).await {
        Ok(db) => db,
        Err(err) => panic!("Could not connect to DB! {}", err),
    }
}
