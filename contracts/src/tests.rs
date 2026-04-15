#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_ticket() {
        let ticket = Ticket {
            id: String::from("TICKET001"),
            event_name: String::from("Metallica Concert"),
            owner: String::from("user123"),
            price: 10000,
            is_valid: true,
        };
        
        assert_eq!(ticket.id, String::from("TICKET001"));
        assert!(ticket.is_valid);
    }

    #[test]
    fn test_transfer_ticket() {
        let original_ticket = Ticket {
            id: String::from("TICKET001"),
            event_name: String::from("Metallica Concert"),
            owner: String::from("user123"),
            price: 10000,
            is_valid: true,
        };
        
        let mut transferred = original_ticket.clone();
        transferred.owner = String::from("user456");
        
        assert_eq!(transferred.owner, String::from("user456"));
        assert_eq!(transferred.id, String::from("TICKET001"));
    }

    #[test]
    fn test_validate_ticket() {
        let ticket = Ticket {
            id: String::from("TICKET001"),
            event_name: String::from("Metallica Concert"),
            owner: String::from("user123"),
            price: 10000,
            is_valid: true,
        };
        
        assert!(ticket.is_valid);
    }

    #[test]
    fn test_revoke_ticket() {
        let mut ticket = Ticket {
            id: String::from("TICKET001"),
            event_name: String::from("Metallica Concert"),
            owner: String::from("user123"),
            price: 10000,
            is_valid: true,
        };
        
        ticket.is_valid = false;
        assert!(!ticket.is_valid);
    }
}