//! Simplistic model layer
use std::sync::{Mutex, Arc};

use serde::{Serialize, Deserialize};

use crate::{Result, Error};


// region: --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    id: u64,
    title: String
}
// we have different types for payloads we need for CRUD
#[derive(Deserialize)]
pub struct TicketForCreate {
    title: String
}

// endregion: --- ticket Types


// region: --- Model Controller
// Clone only clones the Arc
#[derive(Clone)] 
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default()
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id, 
            title: ticket_fc.title
        };

        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_ticket(&self) -> Result<Vec<Ticket>> {
        let mut store = self.tickets_store.lock().unwrap();
        
        // clone the option and its content
        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());

        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}

// endregion: --- Model Controller