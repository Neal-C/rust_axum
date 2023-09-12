//! Mock stuff
//! Not serious

use crate::{appctx::AppCtx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// --- Tickets type
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub creator_id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketCreatePayload {
    pub title: String,
}
// --- Tickets type

// --- Model Controller -> DB connection or Sqlx

#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(
        &self,
        app_ctx: AppCtx,
        ticket_payload: TicketCreatePayload,
    ) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id: u64 = store.len() as u64;

        let ticket: Ticket = Ticket {
            id,
            creator_id: app_ctx.user_id(),
            title: ticket_payload.title,
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self, _: AppCtx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets: Vec<Ticket> = store
            .iter()
            .filter_map(|ticket| ticket.clone())
            .collect::<Vec<Ticket>>();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _: AppCtx, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|ticket| ticket.take());

        ticket.ok_or(Error::TicketDeleteFailedIdNotFound { id })
    }
}
