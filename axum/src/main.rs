use axum::{
    Router,
    body::Bytes,
    extract::{DefaultBodyLimit, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, put},
};
use entities::ApiState;
use entities::SongParams;
use std::env;

mod entities;
mod s3_connector;

#[tokio::main]
async fn main() {
    let s3_client = s3_connector::init_client().await.expect("Environment variables were not set correctly. Please visit the AWS website about 'Using environment variables to globally configure AWS SDKs and tools'.");
    let api_state = ApiState { s3_client };

    println!(
        "Successfully connected to {}.",
        env::var("AWS_ENDPOINT_URL").unwrap()
    );

    let app: Router<()> = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/song", put(put_song).get(get_song))
        .layer(DefaultBodyLimit::max(50 * 1024 * 1024))
        .with_state(api_state);

    let listener = tokio::net::TcpListener::bind(
        env::var("AXUM_LISTENING_SOCKET")
            .expect("AXUM_LISTENING_SOCKET environment variable was not set."),
    )
    .await
    .unwrap();

    println!(
        "Axum API listening on http://{}.",
        env::var("AXUM_LISTENING_SOCKET").unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn get_song(
    State(state): State<ApiState>,
    Query(params): Query<SongParams>,
) -> impl IntoResponse {
    match s3_connector::get_song(&params, &state.s3_client).await {
        Ok(buffer) => {
            let bytes = Bytes::from(buffer);

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "audio/mpeg")
                .header(
                    "Content-Disposition",
                    format!("attachement; filename=\"{}\"", params.build_filename()),
                )
                .body(bytes.into())
                .unwrap()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
async fn put_song(
    State(state): State<ApiState>,
    Query(params): Query<SongParams>,
    body: Bytes,
) -> impl IntoResponse {
    match s3_connector::put_song(&params, body, &state.s3_client).await {
        Ok(..) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
