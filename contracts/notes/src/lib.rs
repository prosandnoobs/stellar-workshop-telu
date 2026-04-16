#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec};

// Data structure to store concert tickets
#[contracttype]
#[derive(Clone, Debug)]
pub struct Ticket {
    pub id: u64,
    pub buyer: Address,       // The wallet address of the ticket owner
    pub concert_name: String, // e.g., "Coldplay Jakarta 2026"
    pub seat: String,         // e.g., "VIP-A12"
}

// Storage key for the ticket data
const TICKET_DATA: Symbol = symbol_short!("TICKETS");

#[contract]
pub struct ConcertTicketContract;

#[contractimpl]
impl ConcertTicketContract {
    // Function to view all sold tickets
    pub fn get_tickets(env: Env) -> Vec<Ticket> {
        env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env))
    }

    // Function to buy a new ticket
    pub fn buy_ticket(env: Env, buyer: Address, concert_name: String, seat: String) -> String {
        // SECURITY: Ensure the buyer actually signed this transaction
        buyer.require_auth(); 

        // 1. Get existing tickets from storage
        let mut tickets: Vec<Ticket> = env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env));

        // 2. Create the new ticket object
        let ticket = Ticket {
            id: env.prng().gen::<u64>(), // Generate random ticket ID
            buyer: buyer,
            concert_name: concert_name,
            seat: seat,
        };

        // 3. Add to the list
        tickets.push_back(ticket);

        // 4. Save back to storage
        env.storage().instance().set(&TICKET_DATA, &tickets);
        
        return String::from_str(&env, "Ticket successfully purchased");
    }

    // Function to cancel/refund a ticket
    pub fn cancel_ticket(env: Env, caller: Address, id: u64) -> String {
        // SECURITY: Ensure the person trying to cancel signed the transaction
        caller.require_auth(); 

        // 1. Get existing tickets from storage
        let mut tickets: Vec<Ticket> = env.storage().instance().get(&TICKET_DATA).unwrap_or(Vec::new(&env));

        // 2. Find and remove the ticket
        for i in 0..tickets.len() {
            let ticket = tickets.get(i).unwrap();
            
            if ticket.id == id {
                // SECURITY: Verify that the person canceling is the actual owner of the ticket
                if ticket.buyer != caller {
                    panic!("Not authorized: You do not own this ticket");
                }

                tickets.remove(i);
                env.storage().instance().set(&TICKET_DATA, &tickets);
                return String::from_str(&env, "Ticket successfully canceled");
            }
        }
        
        return String::from_str(&env, "Ticket not found");
    }
}

mod test;