use std::time::{SystemTime, UNIX_EPOCH};

use crate::{appctx::AppCtx, error::ClientError, Error, Result};
use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

pub struct LogRequest<'service_error> {
    pub uuid: Uuid,
    pub req_method: Method,
    pub uri: Uri,
    pub app_ctx: Option<AppCtx>,
    pub service_error: Option<&'service_error Error>,
    pub client_error: Option<ClientError>,
}

// Push to a CloudWatcher service
pub async fn log_request<'service_error>(log_request: LogRequest<'service_error>) -> Result<()> {
    let timestamp: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let error_type: Option<String> = log_request.service_error.map(|se| se.as_ref().to_string());

    let error_data: Option<Value> = serde_json::to_value(log_request.service_error)
        .ok()
        .and_then(|mut value| value.get_mut("data").map(|value| value.take()));

    let _log_line: RequestLogLine = RequestLogLine {
        uuid: log_request.uuid.to_string(),
        timestamp: timestamp.to_string(),
        req_path: log_request.uri.to_string(),
        req_method: log_request.req_method.to_string(),

        user_id: log_request.app_ctx.map(|ctx| ctx.user_id()),

        client_error_type: log_request.client_error.map(|e| e.as_ref().to_string()),

        error_type,
        error_data,
    };

    Ok(())
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String,      // uuid string formatted
    timestamp: String, // ISO8601 String or Chrono::Date

    // -- User and context attributes
    user_id: Option<u64>,

    //-- http request attributes
    req_path: String,
    req_method: String,

    // -- Errors attributes.
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
