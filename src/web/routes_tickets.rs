use crate::{appctx::AppCtx, Result};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;

use crate::model::{ModelController, Ticket, TicketCreatePayload};

pub fn routes(model_controller: ModelController) -> Router {
    Router::new()
        .route(
            "/tickets",
            get(list_tickets).delete(delete_ticket).post(create_ticket),
        )
        .with_state(model_controller)
}

// --- REST handles
async fn create_ticket(
    app_state: State<ModelController>,
    app_ctx: AppCtx,
    Json(ticket_payload): Json<TicketCreatePayload>,
) -> Result<Json<Ticket>> {
    let model_controller: ModelController = app_state.0;
    let ticket = model_controller
        .create_ticket(app_ctx, ticket_payload)
        .await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(model_controller): State<ModelController>,
    app_ctx: AppCtx,
) -> Result<Json<Vec<Ticket>>> {
    let tickets = model_controller.list_tickets(app_ctx).await?;

    Ok(Json(tickets))
}

#[derive(Deserialize)]
struct ParamId {
    id: u64,
}

async fn delete_ticket(
    State(model_controller): State<ModelController>,
    app_ctx: AppCtx,
    Query(ParamId { id }): Query<ParamId>,
) -> Result<Json<Ticket>> {
    let ticket = model_controller.delete_ticket(app_ctx, id).await?;

    Ok(Json(ticket))
}

// --- REST handlers
