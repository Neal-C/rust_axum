#![allow(clippy::needless_lifetimes)]
#![allow(clippy::needless_return)]

use std::net::SocketAddr;

use appctx::AppCtx;
use axum::{
    extract::Query,
    http::{Method, StatusCode, Uri},
    middleware,
    response::{self, IntoResponse, Response},
    routing, Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

// cargo install cargo-watch
// cargo watch -q -c -w src/ -x run
// quiet clear watch src execute run

mod error;

use crate::{
    error::ClientError,
    log::{log_request, LogRequest},
    model::ModelController,
};

// practice
pub use self::error::{Error, Result};

mod appctx;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let model_controller: ModelController = ModelController::new().await?;

    let api_routes: Router = web::routes_tickets::routes(model_controller.clone())
        .route_layer(middleware::from_fn(web::middleware_auth::mw_require_auth));

    let all_routes: Router = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", api_routes)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            model_controller.clone(),
            web::middleware_auth::mw_appctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // --- Start Server

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("->> LISTENING on {address}\n");

    axum::Server::bind(&address)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();

    // --- Start Server

    Ok(())
}

async fn main_response_mapper(
    app_ctx: Option<AppCtx>,
    uri: Uri,
    request_method: Method,
    response: Response,
) -> Response {
    println!("-->> {:<12} - main_response_mapper", "RESPONSE MAPPER");

    // tracing and debug purpose;
    let uuid: Uuid = Uuid::new_v4();

    // eventual response error
    let service_error: Option<&Error> = response.extensions().get::<Error>(); //access by type

    let client_status_and_error: Option<(StatusCode, ClientError)> =
        service_error.map(|service_error| service_error.client_status_and_error());

    // if Client error, build the new response
    let error_response = client_status_and_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type" : client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            (*status_code, Json(client_error_body)).into_response()
        });

    // build and log the server log line
    let client_error: Option<ClientError> = client_status_and_error.unzip().1;

    // TODO : handle if it failed with match Ok/Err but still won't prevent/stop the request if it did fail
    let _ = log_request(LogRequest {
        uuid,
        uri,
        req_method: request_method,
        app_ctx,
        client_error,
        service_error,
    })
    .await;

    println!();

    return error_response.unwrap_or(response);
}

// --- Handler Routes hello

fn routes_hello() -> Router {
    return Router::new().route("/hello", routing::get(handler_hello));
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello {params:#?}", "HANDLER");

    let name = params.0.name.as_deref().unwrap_or("stranger");

    response::Html(format!("Hello <strong> {name}</strong>"))
}

// --- Handler Routes hello

fn routes_static() -> Router {
    Router::new().nest_service("/", routing::get_service(ServeDir::new("./")))
}
