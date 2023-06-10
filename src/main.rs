use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Extension, Router};
use sqlx::{Executor, MySqlPool, Row};

async fn hello_world(Extension(pool): Extension<MySqlPool>) -> impl IntoResponse {
    let row = match pool.fetch_one("select 1 one").await {
        Ok(row) => row,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };

    match row.try_get::<i32, _>("one") {
        Ok(value) => (StatusCode::OK, value.to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_aws_rds::MySql] pool: MySqlPool) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .layer(Extension(pool));

    Ok(router.into())
}
