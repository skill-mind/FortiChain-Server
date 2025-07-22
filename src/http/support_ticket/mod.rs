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
// mod close_ticket;
// mod unassign_ticket;

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
    //   .route("/close_ticket", post(close_ticket_handler))
    //   .route("/unassign_ticket", post(unassign_ticket_handler))
}
