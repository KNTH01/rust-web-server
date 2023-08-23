pub use self::error::{Error, Result};
use crate::model::ModelController;
use crate::mw::response_mapper::response_mapper_main;
use axum::error_handling::HandleErrorLayer;
use axum::routing::get_service;
use axum::{http, middleware, BoxError, Router};
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod ctx;
mod error;
mod logs;
mod model;
mod mw;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Configure tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "hero_manager_axum=debug,tower_http=debug,sqlx=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING ON {addr}\n");

    let mc = ModelController::new().await?;

    let routes_api = web::routes_todos::routes(mc.clone())
        .route_layer(middleware::from_fn(mw::auth::mw_require_auth));

    let app = Router::new()
        .merge(web::routes_hello::routes())
        .merge(web::routes_login::routes())
        .nest("/api/", routes_api)
        .layer(middleware::map_response(response_mapper_main))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw::auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    http::StatusCode::REQUEST_TIMEOUT
                }))
                .timeout(Duration::from_secs(2)), // .layer(CatchPanicLayer::custom(error::handle_panic)),
        )
        .fallback_service(routes_static());

    info!("listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./public")))
}

