use crate::AppState;
use axum::{
    Router,
    routing::{get, post},
};

mod assign_ticket;
mod domain;
mod list_tickets;
mod open_ticket;
mod resolve_ticket;

pub use domain::*;

pub(crate) fn router() -> Router<AppState> {
    Router::new()
        .route("/open_ticket", post(open_ticket::open_ticket_handler))
        .route("/assign_ticket", post(assign_ticket::assign_ticket_handler))
        .route(
            "/resolve_ticket",
            post(resolve_ticket::resolve_ticket_handler),
        )
        .route("/tickets", get(list_tickets::list_tickets_handler))
        .route("/ticket_history", get(list_tickets::ticket_history_handler))
}
