use core::panic;

use tera::{Context, Tera};

use crate::itemcategory::models::{db_insert_values, ItemCategory};
use axum::{
    extract::Request,
    http::{request, StatusCode},
    response::Html,
    response::IntoResponse,
    Form, Json,
};
use axum_csrf::CsrfToken;
use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, SqlitePool};

const CATEGORY_NAME: &str = "itemcategory";

#[derive(Serialize, Deserialize)]
pub struct Keys {
    authenticity_token: String,
}

pub async fn root(token: CsrfToken) -> impl IntoResponse {
    let token_unwraped = token.authenticity_token().unwrap();

    let tera = Tera::new("templates/**/*").unwrap();

    let mut context = Context::new();
    context.insert("authenticity_token", &token_unwraped);

    let output = tera.render(&(CATEGORY_NAME.to_string() + "/index.html"), &context);

    let db: sqlx::Pool<Sqlite> = crate::db_connection().await;

    let render: Html<String> = Html(output.unwrap());

    (token, render).into_response()
}

pub async fn htmx(token: CsrfToken, Form(payload): Form<Keys>) -> Html<String> {
    if token.verify(&payload.authenticity_token).is_err() {
        println!("Token Invalid");
        panic!()
    } else {
        println!("{}", token.authenticity_token().unwrap());
        let cat: ItemCategory = ItemCategory {
            id: 64,
            name: "nome".to_string(),
            description: "descriçao".to_string(),
        };

        db_insert_values(&cat.name, &cat.description).await;

        let tera = Tera::new("templates/**/*").unwrap();

        let mut context = Context::new();
        context.insert("category", &cat);

        let output = tera.render(
            &(CATEGORY_NAME.to_string() + "/partials/table_row.html"),
            &context,
        );

        Html(output.unwrap())
    }
}
