use core::panic;

use tera::{Context, Tera};

use crate::itemcategory::forms::ItemCategoryForm;
use crate::itemcategory::models::db_insert_values;
use axum::{response::Html, response::IntoResponse, Form};
use axum_csrf::CsrfToken;

const CATEGORY_NAME: &str = "itemcategory";

pub async fn root(token: CsrfToken) -> impl IntoResponse {
    let token_unwraped = token.authenticity_token().unwrap();

    let tera = Tera::new("templates/**/*").unwrap();

    let mut context = Context::new();
    context.insert("authenticity_token", &token_unwraped);

    let output = tera.render(&(CATEGORY_NAME.to_string() + "/index.html"), &context);

    let render: Html<String> = Html(output.unwrap());

    (token, render).into_response()
}

pub async fn htmx(token: CsrfToken, Form(payload): Form<ItemCategoryForm>) -> Html<String> {
    if token.verify(&payload.authenticity_token).is_err() {
        println!("Token Invalid");
        panic!()
    } else {
        println!("Token Valid");
        let mut cat: ItemCategoryForm = payload;

        let id = match db_insert_values(&cat).await {
            Ok(id) => id,
            Err(err) => panic!("Crashed due to {}", err),
        };

        cat.id = id;

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
