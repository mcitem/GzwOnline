use crate::macros::ResultExt;
use crate::{handlers::*, types::AppState};
use axum::{extract::ConnectInfo, http::Request, response::Response, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::request_id::MakeRequestUuid;
use tower_http::ServiceBuilderExt;
use tower_http::{
    compression::{CompressionBody, CompressionLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::Span;

pub async fn new() -> Router {
    let state = AppState {
        db: connect().await,
    };

    let tracing = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            let headers = request.headers();
            let ua = headers
                .get("user-agent")
                .map(|v| v.to_str().unwrap_or("-"))
                .unwrap_or("-");
            let id = headers
                .get("x-request-id")
                .map(|v| v.to_str().unwrap_or("-"))
                .unwrap_or("-");
            let ip = request
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|ConnectInfo(addr)| addr.ip().to_string())
                .unwrap_or_else(|| "-".to_string());
            tracing::span!(
                tracing::Level::INFO,
                "request",
                ip = %ip,
                method = %request.method(),
                uri = %request.uri(),
                v = ?request.version(),
                ua = %ua,
                id = %id,
            )
        })
        .on_response(
            |_response: &Response<CompressionBody<axum::body::Body>>,
             _latency: Duration,
             _: &Span| {
                let _status = _response.status();
                tracing::event!(tracing::Level::INFO, "{} {:?}", _status, _latency);
            },
        );
    Router::new()
        .route("/", get(index::get))
        .nest("/category", category::router())
        .nest("/collection", collection::router())
        .nest("/feedback", feedback::router())
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid::default())
                .layer(tracing)
                .propagate_x_request_id()
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
                .layer(
                    tower_http::cors::CorsLayer::new()
                        .allow_origin("*".parse::<axum::http::HeaderValue>().unwrap())
                        .allow_methods(Any)
                        .allow_headers(vec![axum::http::header::CONTENT_TYPE]),
                )
                .layer(CompressionLayer::new()),
        )
        .with_state(state)
}

async fn connect() -> sea_orm::DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_exit("DATABASE_URL is not set");
    let mut opt = sea_orm::ConnectOptions::new(database_url.to_owned());
    opt.sqlx_logging(false);

    sea_orm::Database::connect(opt)
        .await
        .expect("Failed to connect to database") // panic
}
