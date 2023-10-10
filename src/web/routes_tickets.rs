use crate::{Error, Result};
use axum::{routing::{post, delete}, Router, Json, extract::{State, Path, FromRef}};
use crate::model::{ModelController, Ticket, TicketForCreate};

// axum has a nice way to pass multiple states
// FromRef will make every sub property a sub state that we can inject
#[derive(Clone, FromRef)]
struct AppState {
    // we have sub states
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    // let app_state = AppState { mc };
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_tickets))
        .with_state(mc)
}

// region: --- REST handlers

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_ticket().await?;

    Ok(Json(tickets))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_tickets", "HANDLER");

    let ticket = mc.delete_ticket(id).await?;

    Ok(Json(ticket))
}

// endregion: --- REST handlers
