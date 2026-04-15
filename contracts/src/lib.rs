#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub struct Ticket {
    pub id: String,
    pub event_name: String,
    pub owner: String,
    pub price: i128,
    pub is_valid: bool,
}

#[contract]
pub struct ConcertTicketContract;

#[contractimpl]
impl ConcertTicketContract {
    /// Create a new concert ticket
    pub fn create_ticket(
        env: Env,
        ticket_id: String,
        event_name: String,
        owner: String,
        price: i128,
    ) -> Ticket {
        Ticket {
            id: ticket_id,
            event_name,
            owner,
            price,
            is_valid: true,
        }
    }

    /// Transfer ticket ownership to a new owner
    pub fn transfer_ticket(
        env: Env,
        ticket: Ticket,
        new_owner: String,
    ) -> Ticket {
        let mut updated_ticket = ticket;
        updated_ticket.owner = new_owner;
        updated_ticket
    }

    /// Validate ticket authenticity
    pub fn validate_ticket(env: Env, ticket: Ticket) -> bool {
        ticket.is_valid
    }

    /// Revoke ticket
    pub fn revoke_ticket(env: Env, mut ticket: Ticket) -> Ticket {
        ticket.is_valid = false;
        ticket
    }
}