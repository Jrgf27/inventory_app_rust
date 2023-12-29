use axum::{
    routing::{get, post},
    Router,
};

mod itemcategory;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(itemcategory::views::root))
        .route("/htmx", post(itemcategory::views::htmx));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
