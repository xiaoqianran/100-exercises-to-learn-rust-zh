use crate::data::{Status, Ticket, TicketDraft};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<Mutex<Ticket>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(Mutex::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    // `get` 应返回句柄，调用方可读可改。
    pub fn get(&self, id: TicketId) -> Option<Arc<Mutex<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}
