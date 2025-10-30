use axum::{
    Router,
    body::Bytes,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use entities::song_params::SongParams;
use std::env;

mod entities;
mod s3_connector;

#[tokio::main]
async fn main() {
    let app: Router<()> = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/song", put(put_song).get(get_song));

    let listener = tokio::net::TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("AXUM_LISTENING_PORT")
            .expect("AXUM_LISTENING_URL environment variable was not set")
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_song(Query(params): Query<SongParams>) -> impl IntoResponse {
    match s3_connector::get_song(params).await {
        Ok(..) => StatusCode::OK,
        Err(..) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
async fn put_song(Query(params): Query<SongParams>, body: Bytes) -> impl IntoResponse {
    match s3_connector::put_song(params, body).await {
        Ok(..) => StatusCode::OK,
        Err(..) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
