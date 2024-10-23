mod adapters;
mod application;
mod domain;

use axum::{
    http::{HeaderValue, Method, StatusCode, Uri},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, TraceLayer},
};

use adapters::driving::http::{handlers, middlewares};

pub async fn router() -> Router {
    let app_state = adapters::driving::http::state::AppState::new().await;

    // run migrations
    // run_migrations(&app_state.db_sql_pool).await;

    let public_routes = Router::new();

    let protected_routes = Router::new().route("/files", get(handlers::handler_get_list_files));

    let api = Router::new()
        .nest(
            "/public",
            public_routes
                .layer(Extension(app_state.config.clone()))
                .layer(middleware::from_fn(
                    middlewares::authentication::auth_middleware,
                )),
        )
        .nest("/protected", protected_routes);

    let service_builder = ServiceBuilder::new()
        .layer(middleware::from_fn(
            middlewares::logging::log_request_response,
        ))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(Any)
                .allow_origin([
                    "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                    "http://192.168.1.120:5173".parse::<HeaderValue>().unwrap(),
                    "http://192.168.1.120".parse::<HeaderValue>().unwrap(),
                ]),
        );

    // Return a `Router`
    Router::new()
        .route("/", get(handler_get_root))
        .nest("/api", api)
        .layer(service_builder)
        .fallback(handler_404)
        .with_state(app_state)
}

// root handlers
async fn handler_404(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

async fn handler_get_root() -> &'static str {
    "ok"
}
