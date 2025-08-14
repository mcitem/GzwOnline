use axum::Router;
use dotenv::dotenv;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::writer::MakeWriterExt;
#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting API server...");
    let _g = trace();
    let api = ::api::router::new().await;
    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new("./static"));
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let tcp = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("listening on {}", addr);
    axum::serve(tcp, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

fn trace() -> Option<WorkerGuard> {
    let trac = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false);

    match env("LOG_WRITER") {
        true => {
            let f = tracing_appender::rolling::hourly(".", "log");
            let (n, _g) = tracing_appender::non_blocking(f);

            trac.with_ansi(false)
                .with_writer(n.and(std::io::stdout))
                .init();
            Some(_g)
        }
        false => {
            trac.init();
            None
        }
    }
}

fn env(key: &str) -> bool {
    match std::env::var(key).unwrap_or("false".into()).as_str() {
        "TRUE" => true,
        "True" => true,
        "true" => true,
        "1" => true,
        _ => false,
    }
}
