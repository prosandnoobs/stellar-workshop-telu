#[cfg(test)]
use super::*;

#[test]
fn test_ticket_creation() {
    let ticket = Ticket::new(1, String::from("Concert A"), String::from("Alice"), 100);
    assert_eq!(ticket.id, 1);
    assert_eq!(ticket.concert_name, "Concert A");
    assert_eq!(ticket.owner, "Alice");
    assert_eq!(ticket.price, 100);
}