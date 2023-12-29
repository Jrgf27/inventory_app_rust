use tera::{Context, Tera};

use axum::{http::StatusCode, response::Html, Json};
use serde::{Deserialize, Serialize};

const CATEGORY_NAME: &str = "itemcategory";

#[derive(Serialize)]
struct ItemCategory {
    name: String,
    description: String,
}

pub async fn root() -> Html<String> {
    let tera = Tera::new("templates/**/*").unwrap();

    let context = Context::new();

    let output = tera.render(&(CATEGORY_NAME.to_string() + "/index.html"), &context);

    Html(output.unwrap())
}

pub async fn htmx() -> Html<String> {
    let cat = ItemCategory {
        name: "nome".to_string(),
        description: "descriçao".to_string(),
    };

    let tera = Tera::new("templates/**/*").unwrap();

    let mut context = Context::new();
    context.insert("category", &cat);

    let output = tera.render(
        &(CATEGORY_NAME.to_string() + "/partials/table_row.html"),
        &context,
    );

    Html(output.unwrap())
}
